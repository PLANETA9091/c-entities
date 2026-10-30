#!/usr/bin/env python3
"""Find the first PH-hash divergence tick per scenario leg pair from a parity job log."""
import re, sys

LOG = sys.argv[1] if len(sys.argv) > 1 else "/tmp/parity_fail.log"
lines = open(LOG, encoding="utf-8", errors="replace").read().splitlines()

# leg boundaries by '=== parity leg X (module=Y) ==='
legs = []  # (name, module, start_line_idx)
for i, ln in enumerate(lines):
    m = re.search(r"=== parity leg (\S+) \(module=(\w+)\) ===", ln)
    if m:
        legs.append((m.group(1), m.group(2), i))
if not legs:
    sys.exit("no legs found")

bounds = []
for k, (name, mod, st) in enumerate(legs):
    end = legs[k + 1][2] if k + 1 < len(legs) else len(lines)
    bounds.append((name, mod, st, end))

ph_re = re.compile(r"PH t=(\d+)(?: ct=(-?\d+))? h=([0-9a-f]+) n=(\d+) adds=(\d+)")
sr_re = re.compile(r"SR t=(\d+) s=([0-9a-f]+)")
lr_re = re.compile(r"LR t=(\d+) a=([0-9a-f]+)")

def extract(st, end):
    ph, sr, lr = {}, {}, {}
    for ln in lines[st:end]:
        m = ph_re.search(ln)
        if m:
            t = int(m.group(1))
            ct = int(m.group(2)) if m.group(2) is not None else -1
            ph[t] = (ct, m.group(3), int(m.group(4)), int(m.group(5)))
            continue
        m = sr_re.search(ln)
        if m:
            sr[int(m.group(1))] = m.group(2)
            continue
        m = lr_re.search(ln)
        if m:
            lr[int(m.group(1))] = m.group(2)
    return ph, sr, lr

for k in range(0, len(bounds) - 1, 2):
    name_off, _, s1, e1 = bounds[k]
    name_on, _, s2, e2 = bounds[k + 1]
    sc = name_off.split("-")[1] if "-" in name_off else name_off
    ph1, sr1, lr1 = extract(s1, e1)
    ph2, sr2, lr2 = extract(s2, e2)
    ts = sorted(set(ph1) & set(ph2))
    print(f"\n=== {sc}: off={name_off} ({len(ph1)} ticks) on={name_on} ({len(ph2)} ticks), common={len(ts)}")
    first = None
    for t in ts:
        if ph1[t][1] != ph2[t][1]:
            first = t
            break
    if first is None:
        print("  PH hashes IDENTICAL on all common ticks")
        # check n= and adds= mismatches even if hash same
        for t in ts:
            if ph1[t] != ph2[t]:
                print(f"  (fields differ but hash equal at t={t}: off={ph1[t]} on={ph2[t]})")
        continue
    print(f"  FIRST PH DIVERGENCE at t={first}")
    for t in ts:
        if t < first - 6:
            continue
        if t > first + 12:
            break
        mark = "  <<<" if ph1[t][1] != ph2[t][1] else ""
        print(f"    t={t:5d} off(ct={ph1[t][0]:4d} h={ph1[t][1]} n={ph1[t][2]:5d} adds={ph1[t][3]:4d}) "
              f"on(ct={ph2[t][0]:4d} h={ph2[t][1]} n={ph2[t][2]:5d} adds={ph2[t][3]:4d}){mark}")
    # SR/LR divergence
    fsr = next((t for t in sorted(set(sr1) & set(sr2)) if sr1[t] != sr2[t]), None)
    flr = next((t for t in sorted(set(lr1) & set(lr2)) if lr1[t] != lr2[t]), None)
    print(f"  first SR divergence: t={fsr}, first LR divergence: t={flr}")
