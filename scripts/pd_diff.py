#!/usr/bin/env python3
"""pd_diff.py <dumplog> <tickA> <tickB> [scenario] — per-entity PD diff
across the divergence boundary, split by legs.

PD lines: 'PD t=<tick> id:hex id:hex ...' (10 pairs/line, one m per
entity — the SAME avalanche the total PH sums). The diff at the
divergence tick names the culprit ENTITY ids: which ids vanished
(merge-absorbed/despawned), which appeared (spawn/absorber growth —
same id, different m), per leg.
"""
import re
import sys
from collections import defaultdict


def main():
    path, ta, tb = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    only = sys.argv[4] if len(sys.argv) > 4 else None
    marker = re.compile(r"=== parity leg parity-(\w+)-(off|on) ")
    ph = re.compile(r"\bPH t=(\d+) ct=(-?\d+)")
    pd = re.compile(r"\bPD t=(\d+) ((?:\d+:[0-9a-f]{16}(?: |$))+)")
    leg = None
    prev = -1
    rows = {}  # (leg, t) -> {id: m}
    for line in open(path, encoding="utf-8", errors="replace"):
        m = marker.search(line)
        if m:
            leg = (m.group(1), m.group(2))
            prev = -1
            continue
        pm = ph.search(line)
        if pm:
            t = int(pm.group(1))
            if leg is None or t < prev:
                idx = sum(1 for k in rows if k[0][0].startswith("seg"))
                leg = (f"seg{idx}", "off" if idx % 2 == 0 else "on")
            prev = t
            continue
        dm = pd.search(line)
        if dm:
            t = int(dm.group(1))
            if leg is None or t < prev:
                idx = sum(1 for k in rows if k[0][0].startswith("seg"))
                leg = (f"seg{idx}", "off" if idx % 2 == 0 else "on")
            prev = t
            d = rows.setdefault((leg, t), {})
            for pair in dm.group(2).split():
                i, hv = pair.split(":")
                d[int(i)] = hv

    # group legs by scenario: for each scenario with an off stream,
    # diff against the on stream (marker logs). segN legs pair in order.
    offs = sorted({leg[0] for (leg, t) in rows
                   if leg[1] == "off" and not str(leg[0]).startswith("seg")})
    pairs = [((sc, "off"), (sc, "on")) for sc in offs]
    segs = sorted({leg[0] for (leg, t) in rows if str(leg[0]).startswith("seg")},
                  key=lambda s: int(s[3:]))
    i = 0
    while i + 1 < len(segs):
        pairs.append(((segs[i], "off"), (segs[i + 1], "on")))
        i += 2

    for a, b in pairs:
        sc = a[0]
        if only and sc != only and not str(sc).startswith("seg"):
            continue
        for t in (ta, tb):
            ra, rb = rows.get((a, t), {}), rows.get((b, t), {})
            gone = sorted(set(ra) - set(rb))
            new = sorted(set(rb) - set(ra))
            chg = sorted(k for k in set(ra) & set(rb) if ra[k] != rb[k])
            print(f"[{sc}] t={t} leg-diff: off_n={len(ra)} on_n={len(rb)} "
                  f"vanish={len(gone)} appear={len(new)} changed={len(chg)}")
            if gone:
                print(f"    vanish(off-only): {gone[:24]}")
            if new:
                print(f"    appear(on-only): {new[:24]}")
            if chg:
                print(f"    changed m: {[(k, ra[k][:8], rb[k][:8]) for k in chg[:24]]}")


if __name__ == "__main__":
    main()
