#!/usr/bin/env python3
"""ph_pair_diff.py <off.log> <on.log> — diff PD per-entity dumps of two parity legs."""
import re
import sys
from collections import defaultdict

MASK = (1 << 64) - 1


def ph_mix(k: int) -> int:
    k = (k * 0x9E3779B97F4A7C15) & MASK
    k ^= k >> 30
    k = (k * 0xBF58476D1CE4E5B9) & MASK
    k ^= k >> 27
    k = (k * 0x94D049BB133111EB) & MASK
    return k ^ (k >> 31)


PH = re.compile(r"PH t=(\d+) h=([0-9a-f]+) n=(\d+) adds=(\d+)")
PD = re.compile(r"PD t=(\d+) (.*)$")


def parse(path):
    data = defaultdict(dict)   # tick -> {id: m or None}
    hashes = {}
    with open(path, "r", errors="replace") as f:
        for line in f:
            mp = PH.search(line)
            if mp and " PD " not in line:
                hashes[int(mp.group(1))] = (int(mp.group(2), 16), int(mp.group(3)))
                continue
            mp = PD.search(line)
            if mp:
                t = int(mp.group(1))
                d = data.setdefault(t, {})
                for tok in mp.group(2).strip().split():
                    if ":" in tok:
                        i, mh = tok.split(":", 1)
                        d[int(i)] = int(mh, 16)
                    else:
                        d[int(tok)] = None
    return data, hashes


def main():
    off_data, off_h = parse(sys.argv[1])
    on_data, on_h = parse(sys.argv[2])
    ticks = sorted(set(off_data) & set(on_data))
    print(f"dumped ticks: {ticks[0]}..{ticks[-1]} ({len(ticks)} ticks)")
    # 1. integrity: re-sum vs PH hash
    bad_int = 0
    for t in ticks:
        for tag, d, h in (("off", off_data[t], off_h.get(t)),
                          ("on", on_data[t], on_h.get(t))):
            if h is None or not d:
                continue
            s = 0
            for m in d.values():
                if m is None:
                    s = None
                    break
                s = (s + ph_mix(m)) & MASK
            if s is not None and s != h[0]:
                bad_int += 1
                if bad_int <= 3:
                    print(f"  INTEGRITY {tag} t={t}: re-sum={s:016x} ph={h[0]:016x}")
    if bad_int == 0:
        print("integrity: re-sum == PH hash for all ticks both legs (OK)")
    # 2. first divergence
    for t in ticks:
        off, on = off_data[t], on_data[t]
        missing = sorted(set(off) - set(on))
        extra = sorted(set(on) - set(off))
        state = sorted(i for i in set(off) & set(on) if off[i] != on[i])
        if missing or extra or state:
            print(f"FIRST PD DIVERGENCE t={t}: "
                  f"missing={len(missing)} extra={len(extra)} state={len(state)}")
            for i in missing[:12]:
                print(f"  MISSING-in-on id={i} off_m={off[i]:016x}")
            for i in extra[:12]:
                print(f"  EXTRA-in-on  id={i} on_m={on[i]:016x}")
            for i in state[:12]:
                print(f"  STATE-DIFF  id={i} off={off[i]:016x} on={on[i]:016x}")
            # context: previous tick row for the first culprit
            ctx = t - 1
            if ctx in off_data:
                oc, nc = off_data[ctx], on_data[ctx]
                allids = sorted(set(missing) | set(extra) | set(state))[:12]
                for i in allids[:6]:
                    print(f"  ctx t={ctx} id={i}: off_m={oc.get(i)!r} on_m={nc.get(i)!r}")
            return t
    print("PD identical across the whole dump window")


if __name__ == "__main__":
    main()
