#!/usr/bin/env python3
"""Print aggregated caller stacks ABOVE a callee frame in a JFR.

Usage: stack_above.py <rec.jfr> <callee-substring> [depth] [top-n]
Frame 0 in jfr print stacks is the LEAF; callers come after it.
For each ExecutionSample containing the callee (first occurrence),
take up to `depth` caller frames and count the tuple.
"""
import subprocess
import sys
from collections import Counter


def main():
    jfr, callee = sys.argv[1], sys.argv[2]
    depth = int(sys.argv[3]) if len(sys.argv) > 3 else 5
    topn = int(sys.argv[4]) if len(sys.argv) > 4 else 10
    proc = subprocess.run(
        ["jfr", "print", "--events", "jdk.ExecutionSample", jfr],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr[:400])
    total = 0
    hit = 0
    agg = Counter()
    cur = []
    in_frames = False

    def flush():
        nonlocal total, hit
        if cur:
            total += 1
            for i, fr in enumerate(cur):
                if callee in fr:
                    hit += 1
                    above = tuple(cur[i + 1:i + 1 + depth])
                    agg[above or ("<root>",)] += 1
                    break

    for line in proc.stdout.splitlines():
        s = line.strip()
        if s.startswith("jdk.ExecutionSample"):
            flush()
            cur = []
            in_frames = False
        elif s == "stackTrace = [":
            in_frames = True
        elif s == "]":
            in_frames = False
        elif in_frames and s and s != "..." and "line:" in s:
            cur.append(s.split(" line: ")[0])
    flush()
    print(f"samples={total} hits={hit} ({100.0*hit/total:.1f}%)")
    for k, v in agg.most_common(topn):
        print(f"  {100.0*v/hit:5.1f}%  {' <- '.join(k)}")


if __name__ == "__main__":
    main()
