#!/usr/bin/env python3
"""Print the immediate CALLER frames above a chosen callee in a JFR.

Usage: callers_of.py <rec.jfr> <callee-substring> [top-n]
Counts execution samples whose stack contains the callee and records the
frame directly ABOVE it (its caller). Top-n table with % of all samples.
"""
import subprocess
import sys
from collections import Counter


def main():
    jfr, callee = sys.argv[1], sys.argv[2]
    topn = int(sys.argv[3]) if len(sys.argv) > 3 else 12
    proc = subprocess.run(
        ["jfr", "print", "--events", "jdk.ExecutionSample", jfr],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr[:400])
    total = 0
    hit = 0
    callers = Counter()
    cur = []
    for line in proc.stdout.splitlines():
        if line.strip().startswith("jdk.ExecutionSample"):
            if cur:
                total += 1
                for i, fr in enumerate(cur):
                    if callee in fr:
                        hit += 1
                        # caller = next frame ABOVE (i-1); if callee is the
                        # leaf (i==0) mark as "<leaf>"
                        callers[cur[i - 1].strip() if i > 0 else "<leaf>"] += 1
                        break
            cur = []
        elif line.strip() and "stackTrace" not in line and "thread" not in line.lower():
            cur.append(line.strip())
    if cur:
        total += 1
        for i, fr in enumerate(cur):
            if callee in fr:
                hit += 1
                callers[cur[i - 1].strip() if i > 0 else "<leaf>"] += 1
                break
    print(f"samples={total} hits={hit} ({hit / max(total, 1) * 100:.2f}%)")
    for fr, n in callers.most_common(topn):
        print(f"  {n / max(total, 1) * 100:6.2f}%  {n:6d}  {fr[:130]}")


main()
