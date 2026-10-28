#!/usr/bin/env python3
"""sr_diff.py <joblog> — diff SR/LR entropy-stream lines between parity legs."""
import re
import sys

SR = re.compile(r"SR t=(\d+) s=([0-9a-f]+)")
LR = re.compile(r"LR t=(\d+) a=([0-9a-f]+)")


def parse(path):
    legs = {}
    cur = None
    with open(path, errors="replace") as f:
        for line in f:
            m = re.search(r"=== parity leg parity-(\w+)-(off|on)", line)
            if m:
                cur = m.group(1) + "-" + m.group(2)
                legs.setdefault(cur, {"SR": {}, "LR": {}})
                continue
            if cur is None:
                continue
            m = SR.search(line)
            if m:
                legs[cur]["SR"][int(m.group(1))] = m.group(2)
                continue
            m = LR.search(line)
            if m:
                legs[cur]["LR"][int(m.group(1))] = m.group(2)
    return legs


def main():
    legs = parse(sys.argv[1])
    # group legs by scenario
    scn = {}
    for name in legs:
        s, m = name.rsplit("-", 1)
        scn.setdefault(s, {})[m] = legs[name]
    for s in sorted(scn):
        modes = scn[s]
        if "off" not in modes or "on" not in modes:
            continue
        for stream in ("SR", "LR"):
            off, on = modes["off"][stream], modes["on"][stream]
            ticks = sorted(set(off) & set(on))
            if not ticks:
                print(f"{s} {stream}: no lines")
                continue
            first = None
            for t in ticks:
                if off[t] != on[t]:
                    first = t
                    break
            if first is None:
                print(f"{s} {stream}: identical through t={ticks[-1]} "
                      f"({len(ticks)} ticks)")
            else:
                print(f"{s} {stream}: FIRST DESYNC t={first}")
                for ctx in range(max(ticks[0], first - 2), first + 1):
                    m = "SAME" if off.get(ctx) == on.get(ctx) else "DIFF"
                    print(f"   t={ctx} {m} off={off.get(ctx)} on={on.get(ctx)}")


if __name__ == "__main__":
    main()
