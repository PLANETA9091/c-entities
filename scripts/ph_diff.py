#!/usr/bin/env python3
"""Diff the PARITY PHASH streams (PH t=... lines) between off/on legs
per scenario, from a GitHub Actions job log.

The parity workflow runs legs sequentially with markers:
  === parity leg parity-<sc>-<off|on> (module=...) ===
When the markers are missing (a truncated/streamed step log), the legs
are segmented by the TICK RESET rule: a PH line whose tick is <= the
previous tick starts a new leg segment (each leg restarts the world).
Each leg logs 'PH t=<tick> [ct=<n>] h=<hex> n=<count> adds=<count>'
per tick (ct= joined in df2d659).
This script splits the log, extracts the PH stream per leg, and reports
the FIRST tick where h/n/adds differ per scenario.

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
    # both formats: 'PH t=N h=..' (pre-df2d659) and 'PH t=N ct=M h=..'
    ph = re.compile(r"\bPH t=(\d+)(?: ct=(-?\d+))? h=([0-9a-f]+) n=(\d+) adds=(\d+)")
    prev_tick = -1

    with open(path, encoding="utf-8", errors="replace") as f:
        for line in f:
            m = marker.search(line)
            if m:
                leg = (m.group(1), m.group(2))
                if leg not in streams:
                    streams[leg] = {}
                    order.append(leg)
                # the marker is authoritative: restart the tick baseline so
                # the new leg's ascending t=1.. does NOT trip the reset rule
                prev_tick = -1
                continue
            pm = ph.search(line)
            if pm:
                t = int(pm.group(1))
                if leg is None or t <= prev_tick:
                    # marker missing (streamed/truncated log) OR a world
                    # restart inside the current leg section: start a new
                    # segment keyed by an index suffix (the scenario/leg
                    # pairing is recovered below from the off/on alternation)
                    idx = len(order)
                    leg = (f"seg{idx}", "off" if idx % 2 == 0 else "on")
                    streams.setdefault(leg, {})
                    order.append(leg)
                prev_tick = t
                streams[leg][t] = (
                    pm.group(3), pm.group(4), pm.group(5), line.strip()[-70:],
                )

    scenarios = sorted({sc for sc, _ in order if not sc.startswith("seg")})
    seg_scenarios = []
    if not scenarios:
        # segmented mode: pair the segments in order (off,on) and name them
        # scenario 1..N (the log carried no markers)
        i = 0
        while i + 1 < len(order):
            a, b = order[i], order[i + 1]
            i += 2
            seg_scenarios.append((f"scenario{i // 2}", a, b))
    for sc in scenarios:
        if only and sc != only:
            continue
        off = streams.get((sc, "off"), {})
        on = streams.get((sc, "on"), {})
        if not off or not on:
            print(f"{sc}: MISSING streams (off={len(off)} on={len(on)} lines)")
            continue
        first = None
        nd = 0
        ticks = sorted(set(off) | set(on))
        for t in ticks:
            o = off.get(t)
            n = on.get(t)
            # compare state fields only (h, n, adds) — the raw line embeds
            # wallclock timestamps that differ by construction
            os_ = o[:3] if o else None
            ns_ = n[:3] if n else None
            if os_ != ns_:
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
    for name, a, b in seg_scenarios:
        if only and name != only:
            continue
        off, on = streams.get(a, {}), streams.get(b, {})
        if not off or not on:
            print(f"{name}: MISSING streams (off={len(off)} on={len(on)} lines)")
            continue
        first = None
        nd = 0
        ticks = sorted(set(off) | set(on))
        for t in ticks:
            o = off.get(t)
            n = on.get(t)
            os_ = o[:3] if o else None
            ns_ = n[:3] if n else None
            if os_ != ns_:
                nd += 1
                if first is None:
                    first = (t, o, n)
        if first is None:
            print(f"{name}: IDENTICAL across all {len(ticks)} ticks")
        else:
            t, o, n = first
            print(f"{name}: FIRST DIVERGENCE t={t}  ({nd} diverging ticks of {len(ticks)})")
            print(f"    a: {o[3] if o else '<missing>'}")
            print(f"    b: {n[3] if n else '<missing>'}")


if __name__ == "__main__":
    main()
