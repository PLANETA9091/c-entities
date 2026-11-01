#!/usr/bin/env python3
"""The evidence-run analyzer (addendum 16 dispatches).

Usage: evidence_ana.py <run-artifacts-dir>

Expects the parity-results artifact tree unpacked at <dir>/ with
parity-<scenario>-{off,on}/results/server.log members. Produces:
  1. the per-tick PD diff in the ph_dump window -> the first divergence
     tick + the divergent cohort per scenario;
  2. the TL stream pairs at those ticks -> WHICH tick-list positions
     moved first (the push-integration witness);
  3. the MSIB x MSR pairing at the divergence-window MSQ scans ->
     idxMin vs liveMin per reject id (the freshness class A-D).
"""
import re
import sys
import collections
import os

def load_lines(path):
    if not os.path.exists(path):
        return []
    return open(path, errors='replace').read().splitlines()

def load_pd(lines):
    pd = collections.defaultdict(dict)
    for line in lines:
        m = re.search(r'PD t=(\d+) ', line)
        if m:
            t = int(m.group(1))
            for em in re.finditer(r'(\d+):([0-9a-f]{16})', line):
                pd[t][int(em.group(1))] = em.group(2)
    return pd

def load_tl(lines):
    tl = collections.defaultdict(list)
    for line in lines:
        m = re.search(r'\bTL t=(\d+) (.*)$', line)
        if m:
            tl[int(m.group(1))].append(m.group(2).strip())
    return tl

def load_msib_msr(lines):
    msib = collections.defaultdict(dict)
    msr = collections.defaultdict(dict)
    for line in lines:
        m = re.search(r'MSIB t=(\d+) self=(\d+) rej=(\d+) idxMin=(.*)$', line)
        if m:
            t, self_id = int(m.group(1)), int(m.group(2))
            for em in re.finditer(r'(\d+):\[(\([^)]*\)|X|[-0-9.eE,]+)\]', m.group(4)):
                msib[(t, self_id)][int(em.group(1))] = em.group(2)
        m = re.search(r'MSR t=(\d+) self=(\d+) rej=(\d+) liveMin=(.*)$', line)
        if m:
            t, self_id = int(m.group(1)), int(m.group(2))
            for em in re.finditer(r'(\d+):\[([-0-9.eE,]+)\]', m.group(4)):
                msr[(t, self_id)][int(em.group(1))] = em.group(2)
    return msib, msr

def main(root):
    scenarios = ('mobs', 'spread', 'mixed')
    for scen in scenarios:
        off = load_lines(f'{root}/parity-{scen}-off/results/server.log')
        on = load_lines(f'{root}/parity-{scen}-on/results/server.log')
        pd_off, pd_on = load_pd(off), load_pd(on)
        ticks = sorted(set(pd_off) & set(pd_on))
        divs = []
        for t in ticks:
            bad = [eid for eid in set(pd_off[t]) | set(pd_on[t])
                   if pd_off[t].get(eid) != pd_on[t].get(eid)]
            if bad:
                divs.append((t, bad))
        if ticks:
            print(f"== {scen}: PD window {ticks[0]}..{ticks[-1]}, "
                  f"div ticks={len(divs)}, first={divs[0][0] if divs else None}")
            if divs:
                t0, bad0 = divs[0]
                print(f"   first-div cohort ({len(bad0)}): {sorted(bad0)[:12]}")
        # TL pairs at the first-div tick
        tl_off, tl_on = load_tl(off), load_tl(on)
        if divs and (tl_off or tl_on):
            t0 = divs[0][0]
            lo, ln = tl_off.get(t0, []), tl_on.get(t0, [])
            print(f"   TL t={t0}: off_lines={len(lo)} on_lines={len(ln)}")
            for a, b in zip(lo[:4], ln[:4]):
                mark = 'SAME' if a == b else 'DIFF'
                print(f"     [{mark}] off: {a[:110]}")
                print(f"     [{mark}] on : {b[:110]}")
        # MSIB x MSR pairing (ON leg only — the module emits them)
        msib, msr = load_msib_msr(on)
        pairs = 0
        classes = collections.Counter()
        for key in sorted(set(msib) & set(msr)):
            for eid, idx in msib[key].items():
                live = msr[key].get(eid)
                if live is None:
                    continue
                pairs += 1
                if idx == 'X' or idx.startswith('('):
                    classes['D_notindexed'] += 1
                elif idx == live:
                    classes['A_margin'] += 1
                else:
                    # same block? parse the first coord pair
                    try:
                        ix = float(idx.split(',')[0])
                        lx = float(live.split(',')[0])
                        classes['B_or_C_stale' if abs(ix - lx) > 1e-4 else 'A_tiny'] += 1
                    except ValueError:
                        classes['parse'] += 1
        if msib or msr:
            print(f"   MSIB/MSR keys: msib={len(msib)} msr={len(msr)} pairs={pairs} classes={dict(classes)}")

if __name__ == '__main__':
    main(sys.argv[1])
