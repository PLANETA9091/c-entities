#!/usr/bin/env python3
"""Find the first diverging ENTITY via PD (per-entity hash) lines around first PH divergence."""
import re, sys

LOG = sys.argv[1] if len(sys.argv) > 1 else "/tmp/parity_fail.log"
T0 = int(sys.argv[2]) if len(sys.argv) > 2 else 0   # first divergent tick
T1 = int(sys.argv[3]) if len(sys.argv) > 3 else 0   # last tick to scan (inclusive)
SC = sys.argv[4] if len(sys.argv) > 4 else None     # scenario name filter

lines = open(LOG, encoding="utf-8", errors="replace").read().splitlines()
legs = [(m.group(1), i) for i, ln in enumerate(lines)
        for m in [re.search(r"=== parity leg (\S+) \(module=", ln)] if m]
pd_re = re.compile(r"\[ce-baseline\] PD t=(\d+) ((?:\d+:[0-9a-f]+ ?)+)")

def extract(st, end):
    pd = {}
    for ln in lines[st:end]:
        m = pd_re.search(ln)
        if m:
            t = int(m.group(1))
            pd.setdefault(t, {})
            for pair in m.group(2).split():
                eid, h = pair.split(":")
                pd[t][int(eid)] = h
    return pd

for k in range(len(legs) - 1):
    name = legs[k][0]
    if SC and SC not in name:
        continue
    st1, st2 = legs[k][1], legs[k + 1][1]
    end2 = legs[k + 2][1] if k + 2 < len(legs) else len(lines)
    pd1 = extract(st1, st2)
    pd2 = extract(st2, end2)
    a, b = T0 - 8, (T1 or T0 + 8)
    print(f"\n=== {name}: PD diff for t in [{a}..{b}]")
    for t in range(max(1, a), b + 1):
        d1, d2 = pd1.get(t, {}), pd2.get(t, {})
        all_ids = sorted(set(d1) | set(d2))
        diffs = [i for i in all_ids if d1.get(i) != d2.get(i)]
        if diffs:
            print(f"  t={t}: {len(diffs)} diverged ids: {diffs[:14]}")
            for i in diffs[:6]:
                print(f"    id {i}: off={d1.get(i)} on={d2.get(i)}")
            new_ids = [i for i in diffs if i not in pd1.get(t - 1, {}) and i not in pd2.get(t - 1, {})]
            if new_ids:
                print(f"    (fresh spawns this tick: {new_ids[:10]})")
