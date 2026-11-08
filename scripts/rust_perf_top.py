#!/usr/bin/env python3
"""rust_perf_top.py — the Rust per-function self-time report for the
native pivot (L110, doc 42).

Runs the native bench under `perf record` (when perf exists and the
paranoid level allows), parses `perf report` (or `perf script`) and prints
a TOP table of RUST symbols by self-time. Without perf (CI runners often
restrict perf_event_open), it falls back to:
  1. `valgrind --tool=callgrind` (if present) — exact, just slow;
  2. the crate's built-in phase ledger (always available).

Usage:
  rust_perf_top.py [--mobs N] [--ticks N] [--seed S] [--top K] [--out FILE]
"""
import argparse, os, re, shutil, subprocess, sys, tempfile

CARGO = os.environ.get("CARGO", "cargo")
BENCH = "native_bench"


def sh(cmd, **kw):
    print("+", " ".join(cmd), file=sys.stderr)
    return subprocess.run(cmd, capture_output=True, text=True, **kw)


def build():
    r = sh([CARGO, "build", "--release", "-p", "centities-native", "--example", BENCH])
    if r.returncode != 0:
        print(r.stderr[-4000:], file=sys.stderr)
        sys.exit(2)
    return os.path.join(
        "target", "release", "examples", BENCH
    )


def run_perf(binary, args, out):
    r = sh(
        [
            "perf", "record", "-e", "cycles:u", "-o", out, "--",
            binary, str(args.mobs), str(args.ticks), str(args.seed),
        ]
    )
    if r.returncode != 0:
        print("perf record failed:", r.stderr[-2000:], file=sys.stderr)
        return False
    return True


def parse_perf_report(out, top):
    r = sh(["perf", "report", "-i", out, "--stdio", "--no-children",
            "-s", "symbol,dso", "--percent-limit", "0.1"])
    rows = []
    for line in r.stdout.splitlines():
        # 12.34%  centities_core::goals::ai_floor  [centities_native.xxx]
        m = re.match(r"\s*([0-9.]+)%\s+(\S+?)\s*(?:\[.+?\])?\s*$", line)
        if m:
            sym = m.group(2)
            if "::" in sym or sym.startswith("_ZN"):
                rows.append((float(m.group(1)), sym))
        else:
            m2 = re.match(r"\s*([0-9.]+)%\s+(.*)$", line)
            if m2 and "::" in m2.group(2):
                sym = m2.group(2).split()[0]
                rows.append((float(m2.group(1)), sym))
    agg = {}
    for pct, sym in rows:
        agg[sym] = agg.get(sym, 0.0) + pct
    return sorted(agg.items(), key=lambda kv: -kv[1])[:top]


def find_repo():
    env = os.environ.get("CE_REPO")
    if env:
        return env
    d = os.path.dirname(os.path.abspath(__file__))
    for _ in range(4):
        if os.path.isfile(os.path.join(d, "Cargo.toml")):
            return d
        d = os.path.dirname(d)
    return os.getcwd()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--mobs", type=int, default=25000)
    ap.add_argument("--ticks", type=int, default=400)
    ap.add_argument("--seed", type=lambda s: int(s, 0), default=194531)
    ap.add_argument("--top", type=int, default=30)
    ap.add_argument("--out", default=None)
    a = ap.parse_args()

    os.chdir(find_repo())
    binary = build()
    lines = []

    if shutil.which("perf"):
        with tempfile.NamedTemporaryFile(suffix=".perf", delete=False) as f:
            pf = f.name
        try:
            ok = run_perf(binary, a, pf)
            if ok:
                rows = parse_perf_report(pf, a.top)
                if rows:
                    lines.append("## Rust per-function self-time (perf cycles:u)")
                    lines.append("| # | % | function |")
                    lines.append("|---:|---:|---|")
                    for i, (pct, sym) in enumerate(rows, 1):
                        lines.append(f"| {i} | {pct:.2f}% | `{sym}` |")
        finally:
            os.unlink(pf)

    if not lines:
        # fallback: the phase ledger (always available, near-zero cost)
        r = sh([binary, str(a.mobs), str(a.ticks), str(a.seed)])
        lines.append("## Native phase ledger (perf unavailable — built-in sampling)")
        lines.append("```")
        lines.extend(r.stdout.strip().splitlines())
        lines.append("```")

    text = "\n".join(lines)
    print(text)
    if a.out:
        with open(a.out, "w") as f:
            f.write(text + "\n")
        print("written:", a.out, file=sys.stderr)


if __name__ == "__main__":
    main()
