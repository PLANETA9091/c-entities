#!/usr/bin/env python3
"""The no-root Rust profiler: PGO instrumentation -> per-function counters.

The sandbox/CI cannot use `perf` (no root, perf_event_open blocked), so the
per-function view of the native executor comes from LLVM PGO instrumentation
instead: build with -Cprofile-generate, run the bench, merge the .profraw
files, and rank the functions by their internal block counters.

Usage: python3 scripts/rust_pgo_top.py [N_MOBS] [TICKS] [SEED]

Caveat (doc 42 9.1, measured): block COUNT is not TIME — a trivial store
loop can outrank real work. Use this to locate WHERE the executions are,
and the interleaved bench p50 pairs + the phase ledger to attribute time.
"""
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
LLVM = sorted(Path.home().glob(
    ".rustup/toolchains/*/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata"))
LLVM_NM = sorted(Path.home().glob(
    ".rustup/toolchains/*/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-nm"))


def sh(cmd, env=None, quiet=False):
    args = (cmd.split() if isinstance(cmd, str) else [str(c) for c in cmd])
    print("+", " ".join(args))
    r = subprocess.run(args, cwd=REPO, env=env,
                       capture_output=True, text=True)
    if r.returncode != 0 and not quiet:
        print(r.stdout[-2000:], r.stderr[-2000:])
    return r


def main():
    n, ticks, seed = (sys.argv[1:4] if len(sys.argv) > 3 else
                      ("25000", "400", "12648430"))
    profdata = Path("/tmp/bench.profdata")
    pgodir = Path("/tmp/pgo-data")
    if not LLVM:
        sys.exit("llvm-profdata not found: rustup component add llvm-tools-preview")
    # cargo must be on PATH (rustup installs it to ~/.cargo/bin)
    env = dict(os.environ)
    env["RUSTFLAGS"] = f"-Cprofile-generate={pgodir}"
    cargo_bin = Path.home() / ".cargo/bin"
    if cargo_bin.is_dir():
        env["PATH"] = f"{cargo_bin}:{env.get('PATH', '')}"
    if not shutil.which("cargo", path=env["PATH"]):
        sys.exit("cargo not found: install the rust toolchain first")
    pgodir.mkdir(exist_ok=True)
    sh("cargo build --release -p centities-native --example native_bench",
       env=env)
    sh(["target/release/examples/native_bench", n, ticks, seed])
    sh([LLVM[-1], "merge", "-o", profdata] + list(pgodir.glob("*.profraw")))
    out = sh([LLVM[-1], "show", "--all-functions", "--counts", profdata],
             quiet=True).stdout
    funcs = {}
    name = None
    for line in out.splitlines():
        m = re.match(r"^  (\S+):$", line)
        if m:
            name = m.group(1)
            continue
        if name and "Block counts:" in line:
            blocks = [int(x) for x in re.findall(r"\d+", line)]
            if blocks:
                funcs[name] = max(funcs.get(name, 0), max(blocks))
    sizes = {}
    if LLVM_NM:
        for l in sh([LLVM_NM[-1], "-S",
                     "target/release/examples/native_bench"], quiet=True).stdout.splitlines():
            p = l.split()
            if len(p) == 4 and p[3][0] in "tTwW":
                sizes[p[3]] = int(p[1], 16)

    def dm(nm):
        nm = re.sub(r"^[^;]*;", "", nm)
        nm = re.sub(r"-Cs[A-Za-z0-9_]{16}", "", nm)
        nm = re.sub(r"Nt[A-Za-z0-9_]{16}", "", nm)
        return nm.replace("_R", "")[:96]

    rows = [(c * min(max(sizes.get(k.split(";")[-1], 1), 1), 2048), c, dm(k))
            for k, c in funcs.items()]
    rows.sort(reverse=True)
    print(f"\nTOP-{min(40, len(rows))} functions by max internal block count\n")
    print(f"{'weight':>16} {'maxblock':>13}  function")
    for w, c, nm in rows[:40]:
        print(f"{w:>16,} {c:>13,}  {nm}")


if __name__ == "__main__":
    main()
