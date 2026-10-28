#!/usr/bin/env python3
"""Diff the PARITY PHASH streams (PH t=... lines) between off/on legs
per scenario, from a GitHub Actions job log.

The parity workflow runs legs sequentially with markers:
  === parity leg parity-<sc>-<off|on> (module=...) ===
Each leg logs 'PH t=<tick> h=<hex> n=<count> adds=<count>' per tick.
This script splits the log by markers, extracts the PH stream per leg,
and reports the FIRST tick where h/n/adds differ per scenario.

Usage: ph_diff.py <joblog> [scenario-filter]
"""
import re
import sys


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)
    path = sys.argv[1]
    only = sys.argv[2] if len(sys.argv) > 2 else None

    leg = None
    streams = {}  # (scenario, legname) -> {tick: (h, n, adds, raw)}
    order = []
    marker = re.compile(r"=== parity leg parity-(\w+)-(off|on) ")
    ph = re.compile(r"PH t=(\d+) h=([0-9a-f]+) n=(\d+) adds=(\d+)")

    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = marker.search(line)
            if m:
                leg = (m.group(1), m.group(2))
                if leg not in streams:
                    streams[leg] = {}
                    order.append(leg)
                continue
            if leg is None:
                continue
            pm = ph.search(line)
            if pm:
                t = int(pm.group(1))
                streams[leg][t] = (
                    pm.group(2), pm.group(3), pm.group(4), line.strip()[-70:],
                )

    scenarios = sorted({sc for sc, _ in order})
    for sc in scenarios:
        if only and sc != only:
            continue
        off = streams.get((sc, "off"), {})
        on = streams.get((sc, "on"), {})
        if not off or not on:
            print(f"{sc}: MISSING streams (off={len(off)} on={len(on)} lines)")
            continue
        ticks = sorted(set(off) | set(on))
        first = None
        nd = 0
        for t in ticks:
            o = off.get(t)
            n = on.get(t)
            # compare state fields only (h, n, adds) — the raw line embeds
            # wallclock timestamps that differ by construction
            os = o[:3] if o else None
            ns = n[:3] if n else None
            if os != ns:
                nd += 1
                if first is None:
                    first = (t, o, n)
        if first is None:
            print(f"{sc}: IDENTICAL across all {len(ticks)} ticks")
        else:
            t, o, n = first
            print(f"{sc}: FIRST DIVERGENCE t={t}  ({nd} diverging ticks of {len(ticks)})")
            print(f"    off: {o[3] if o else '<missing>'}")
            print(f"    on : {n[3] if n else '<missing>'}")
            tctx = [x for x in ticks if x < t][-3:]
            for tc in tctx:
                print(f"    ctx t={tc}: off={off.get(tc, ('', '', '', '<missing>'))[3]}")
                print(f"             on ={on.get(tc, ('', '', '', '<missing>'))[3]}")


if __name__ == "__main__":
    main()
