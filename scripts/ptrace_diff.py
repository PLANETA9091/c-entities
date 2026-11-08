#!/usr/bin/env python3
"""ptrace_diff.py <joblog> — split PTRACE streams by parity leg marker and diff."""
import re
import sys

off, on = {}, {}
cur = None
with open(sys.argv[1], errors="replace") as f:
    for line in f:
        if "parity leg parity-items-off" in line:
            cur = off
        elif "parity leg parity-items-on" in line:
            cur = on
        elif "parity leg parity-mixed-off" in line:
            cur = None  # extend as needed
        elif "parity leg parity-mixed-on" in line:
            cur = None
        else:
            m = re.search(r"PTRACE t=(\d+) (.*)$", line)
            if m and cur is not None:
                cur[int(m.group(1))] = m.group(2)

lo = int(sys.argv[2]) if len(sys.argv) > 2 else 140
hi = int(sys.argv[3]) if len(sys.argv) > 3 else 200
diffs = 0
for t in sorted(set(off) & set(on)):
    same = off[t] == on[t]
    if not same:
        diffs += 1
        print(f"t={t} DIFF")
        print("  off:", off[t][:200])
        print("  on :", on[t][:200])
    elif lo <= t <= hi:
        print(f"t={t} same")
        print("  off:", off[t][:200])
print("diff ticks:", diffs)
