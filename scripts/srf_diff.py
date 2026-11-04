#!/usr/bin/env python3
"""srf_diff.py <hunt-joblog> — the L85b SRF analyzer.

Pairs the OFF/ON legs of each parity scenario (the `=== parity leg
parity-<scen>-<mod> (module=..) ===` markers) and compares:

  * SRC t=<tick> calls=<n>  — the SHARED_RANDOM per-tick draw counts
    (the L85b wrap: installed at ENABLE, isSr routing, SrUnwrap-safe);
  * SRF t=<tick> k=<i> <frames> — the per-draw caller frames (deepest
    first), inside LRC_WINDOW;
  * SR t=<tick> s=<hex>      — the LCG state probe (the pre-existing);
  * LRC/LRF                  — the Level.random twin;
  * PH t=<tick> h=.. n=..    — the state hash.

Output per scenario: the first SRC-divergent tick, the SR state first
divergence, then the SRF frame diff AT the first SRC divergence tick —
frames present in ON and not in OFF (the extra consumers) and vice
versa, with multiplicities. This names the +17's caller directly.
"""
import re
import sys
from collections import Counter

LEG_RE = re.compile(r"=== parity leg parity-(\w+)-(off|on) \(module=")
SRC_RE = re.compile(r"SRC t=(\d+) calls=(\d+)(?: sc=(\S+))?")
SR_RE = re.compile(r"SR t=(\d+) s=([0-9a-f]+)(?: sc=(\S+))?")
SRF_RE = re.compile(r"SRF t=(\d+) k=(\d+) (.*)$")
LRC_RE = re.compile(r"LRC t=(\d+) calls=(\d+)(?: sc=(\S+))?")
LRF_RE = re.compile(r"LRF t=(\d+) k=(\d+) n=(\d+)")
LRFF_RE = re.compile(r"LRF t=(\d+) k=(\d+) (.*)$")
PH_RE = re.compile(r"PH t=(\d+) ct=(-?\d+) h=([0-9a-f]+) n=(\d+) adds=(-?\d+)")
# L90: the engine's gate ledger (ON leg only — the module prints it):
# GL t=<tick> sc=<tag> <kind>=evals/no/staged/rolls ...
GL_RE = re.compile(r"GL t=(\d+) sc=(\S+)((?: \d+=\d+/\d+/\d+/\d+)+)")
# L90: the vanilla-side per-callsite census (BOTH legs, cap-free):
# GLV t=<tick> sc=<tag> n=<total> <Class.method>=<n> ...
GLV_RE = re.compile(r"GLV t=(\d+) sc=(\S+) n=(\d+)((?: [^ ]+))")
GLV_SITE_RE = re.compile(r"([A-Za-z0-9_$]+\.[A-Za-z0-9_$]+)=(\d+)")
GL_KIND_RE = re.compile(r"(\d+)=(\d+)/(\d+)/(\d+)/(\d+)")


def line_tag(line, cur):
    """L90: attribute the line by its trailing sc= tag when present — the
    8 interleaved consoles share one job log and the LAST marker no
    longer decides. Falls back to the current leg marker."""
    i = line.rfind(" sc=")
    if i >= 0:
        tag = line[i + 4:].split()[0]
        m = LEG_RE.match(tag, 0)
        # tags look like parity-mobs-on — same shape as the leg key
        if tag in LEGS_KNOWN or m:
            return tag
        return tag
    return cur


LEGS_KNOWN = set()


def parse(path):
    legs = {}
    cur = None
    with open(path, "rb") as f:
        for raw in f:
            line = raw.decode("utf-8", "replace")
            m = LEG_RE.search(line)
            if m:
                cur = m.group(1) + "-" + m.group(2)
                LEGS_KNOWN.add(cur)
                legs.setdefault(cur, {"SRC": {}, "SR": {}, "SRF": {},
                                      "LRC": {}, "PH": {}, "LRF": {},
                                      "GL": {}, "GLV": {}})
                continue
            if cur is None:
                continue
            tag = line_tag(line, cur)
            d = legs.setdefault(tag, {"SRC": {}, "SR": {}, "SRF": {},
                                      "LRC": {}, "PH": {}, "LRF": {},
                                      "GL": {}, "GLV": {}})
            m = SRC_RE.search(line)
            if m:
                d["SRC"][int(m.group(1))] = int(m.group(2))
                continue
            m = SRF_RE.search(line)
            if m:
                d["SRF"].setdefault(int(m.group(1)), []).append(m.group(3))
                continue
            m = SR_RE.search(line)
            if m:
                d["SR"][int(m.group(1))] = m.group(2)
                continue
            m = GL_RE.search(line)
            if m:
                tick = int(m.group(1))
                kinds = d["GL"].setdefault(tick, {})
                for km in GL_KIND_RE.finditer(m.group(3)):
                    kinds[int(km.group(1))] = tuple(int(km.group(i)) for i in range(2, 6))
                continue
            m = GLV_RE.search(line)
            if m:
                tick = int(m.group(1))
                sites = d["GLV"].setdefault(tick, Counter())
                for sm in GLV_SITE_RE.finditer(m.group(4)):
                    sites[sm.group(1)] += int(sm.group(2))
                continue
            m = LRC_RE.search(line)
            if m:
                d["LRC"][int(m.group(1))] = int(m.group(2))
                continue
            m = LRFF_RE.search(line)
            if m:
                d["LRF"].setdefault(int(m.group(1)), []).append(m.group(3))
                continue
            m = PH_RE.search(line)
            if m:
                d["PH"][int(m.group(1))] = (m.group(3), int(m.group(4)),
                                            int(m.group(5)))
    return legs


def first_div(a, b, key):
    da, db = a[key], b[key]
    for t in sorted(set(da) | set(db)):
        va = da.get(t)
        vb = db.get(t)
        if va != vb:
            return t, va, vb
    return None, None, None


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    legs = parse(sys.argv[1])
    scen = {}
    for name, d in legs.items():
        s, m = name.rsplit("-", 1)
        scen.setdefault(s, {})[m] = d
    for s in sorted(scen):
        modes = scen[s]
        if "off" not in modes or "on" not in modes:
            print(f"[{s}] legs: {sorted(modes)} — missing pair, skip")
            continue
        off, on = modes["off"], modes["on"]
        print(f"=== {s} ===")
        # sanity: was the wrap live at all?
        src_off = sum(off["SRC"].values())
        src_on = sum(on["SRC"].values())
        print(f"  SRC total: off={src_off} on={src_on}"
              + ("  (WRAP DEAD — calls=0)" if src_off == src_on == 0 else ""))
        for key, label in (("SRC", "SRC calls"), ("SR", "SR state"),
                           ("LRC", "LRC calls"), ("PH", "PH hash")):
            t, vo, vn = first_div(off, on, key)
            if t is None:
                print(f"  {label}: identical everywhere")
            else:
                print(f"  {label}: FIRST DIV at t={t}  off={vo}  on={vn}")
        # SRF diff at the first SRC divergence
        t, vo, vn = first_div(off, on, "SRC")
        if t is not None and (off["SRF"].get(t) or on["SRF"].get(t)):
            fo = Counter(off["SRF"].get(t, []))
            fn = Counter(on["SRF"].get(t, []))
            only_on = fn - fo
            only_off = fo - fn
            print(f"  SRF diff at t={t}: frames only in ON "
                  f"({sum(only_on.values())} draws):")
            for f, c in only_on.most_common(12):
                print(f"    x{c}: {f.strip()}")
            print(f"  frames only in OFF ({sum(only_off.values())} draws):")
            for f, c in only_off.most_common(12):
                print(f"    x{c}: {f.strip()}")
        elif t is not None:
            print(f"  SRF at t={t}: no frames recorded "
                  f"(off={len(off['SRF'].get(t, []))} "
                  f"on={len(on['SRF'].get(t, []))})")
        # L90: the census diff — the OFF leg's GLV is the vanilla truth;
        # the ON leg's GLV shows only the Java-visible draws (the engine
        # serves Rust-side) and its GL ledger carries the Rust rolls.
        for td in sorted(set(off["GLV"]) | set(on["GLV"])):
            fo2 = off["GLV"].get(td, Counter())
            fn2 = on["GLV"].get(td, Counter())
            if fo2 == fn2:
                continue
            keys = sorted(set(fo2) | set(fn2))
            print(f"  GLV diff at t={td} (vanilla-census off vs java-visible on):")
            for k2 in keys:
                a, b = fo2.get(k2, 0), fn2.get(k2, 0)
                if a != b:
                    print(f"    {k2}: off={a} on={b}")
        for td in sorted(on["GL"]):
            kinds = on["GL"][td]
            if kinds:
                s2 = " ".join(f"k{k}=e{v[0]}/n{v[1]}/st{v[2]}/r{v[3]}"
                              for k, v in sorted(kinds.items()))
                print(f"  GL(on) t={td}: {s2}")


if __name__ == "__main__":
    main()
