#!/usr/bin/env python3
"""Aggregate normalized CALL CHAINS whose leaf matches a callee substring.

Usage: chain_of.py <rec.jfr> <callee-substring> [chain-depth] [top-n]

Parses `jfr print --events jdk.ExecutionSample` output with a proper
stackTrace=[ ... ] block state machine. For every sample whose stack
contains <callee-substring>, takes the frames from the leaf up to
<chain-depth> frames above it, joins leaf-last with " <- " and counts.
Prints the top-n chains with % of ALL samples.
"""
import re
import subprocess
import sys
from collections import Counter


def norm(frame: str) -> str:
    f = re.sub(r" line: \d+", "", frame)
    return f.strip()


def main():
    jfr, callee = sys.argv[1], sys.argv[2]
    depth = int(sys.argv[3]) if len(sys.argv) > 3 else 8
    topn = int(sys.argv[4]) if len(sys.argv) > 4 else 14
    proc = subprocess.run(
        ["jfr", "print", "--stack-depth", "64", "--events",
         "jdk.ExecutionSample", jfr],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr[:400])
    total = 0
    chains = Counter()
    cur = []
    in_stack = False
    for line in proc.stdout.splitlines():
        s = line.strip()
        if s.startswith("jdk.ExecutionSample"):
            if cur:
                total += 1
                for i, fr in enumerate(cur):
                    if callee in fr:
                        seg = cur[i:i + 1 + depth]  # leaf first, callers after
                        chains[" <- ".join(norm(f) for f in seg)] += 1
                        break
            cur = []
            in_stack = False
        elif s.startswith("stackTrace = ["):
            in_stack = True
            cur = []
        elif s == "]":
            if in_stack and cur:
                total += 1
                for i, fr in enumerate(cur):
                    if callee in fr:
                        seg = cur[i:i + 1 + depth]
                        chains[" <- ".join(norm(f) for f in seg)] += 1
                        break
            in_stack = False
            cur = []
        elif in_stack and s:
            cur.append(s)
    if cur and in_stack:
        total += 1
        for i, fr in enumerate(cur):
            if callee in fr:
                seg = cur[max(0, i - depth):i + 1]
                chains[" <- ".join(norm(f) for f in reversed(seg))] += 1
                break
    print(f"samples={total} matching-callee={sum(chains.values())}")
    for ch, n in chains.most_common(topn):
        print(f"{n * 100.0 / total:6.2f}%  {ch}")


if __name__ == "__main__":
    main()
