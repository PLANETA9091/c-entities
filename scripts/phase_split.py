#!/usr/bin/env python3
"""Phase-split JFR analysis: split samples by wall-clock window, then
compute markers + caller chains of EntityLookup.getEntities(Class,...)
per phase.

Usage: phase_split.py <rec.jfr> <HH:MM:SS>-<HH:MM:SS> [...]
Window label = its start time.
"""
import subprocess
import sys
import re
from collections import Counter

MARKERS = [
    "mergeWithNeighbours",
    "EntityLookup.getEntities(Class",
    "getEntitiesOfClass",
    "ItemEntity.tick(",
    "Entity.tick(",
    "inactiveTick",
    "moonrise$tick",
    "updatePlayer",
    "EntityTickList.forEach",
    "InsideBlockEffectApplier",
    "queryNative",
    "fastQuery",
    "rowQuery",
    "NaturalSpawner",
    "activateEntities",
    "AABB.intersects",
    "noCollision",
    "CeHooks",
]


class Win:
    def __init__(self, a, b):
        self.a, self.b = a, b
        self.tot = 0
        self.marks = Counter()
        self.callers = Counter()


def main():
    jfr = sys.argv[1]
    wins = []
    for s in sys.argv[2:]:
        a, b = s.split("-")
        wins.append(Win(a, b))
    proc = subprocess.run(
        ["jfr", "print", "--events", "jdk.ExecutionSample", jfr],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr[:400])

    cur = []
    t = None
    in_frames = False

    def flush():
        if t is None:
            return
        for w in wins:
            if w.a <= t <= w.b:
                w.tot += 1
                for m in MARKERS:
                    if any(m in f for f in cur):
                        w.marks[m] += 1
                for i, fr in enumerate(cur):
                    if "EntityLookup.getEntities(Class" in fr:
                        w.callers[tuple(cur[i + 1:i + 5])] += 1
                        break
                break

    for line in proc.stdout.splitlines():
        s = line.strip()
        if s.startswith("jdk.ExecutionSample"):
            flush()
            cur = []
            in_frames = False
            t = None  # startTime arrives on a later line
        elif t is None and s.startswith("startTime = "):
            m = re.search(r"(\d\d:\d\d:\d\d)", s)
            t = m.group(1) if m else None
        elif s == "stackTrace = [":
            in_frames = True
        elif s == "]":
            in_frames = False
        elif in_frames and s and s != "...":
            cur.append(s.split(" line: ")[0])
    flush()

    print(f"jfr: {jfr}")
    for w in wins:
        print(f"\n=== window {w.a}..{w.b}: {w.tot} samples ===")
        for m in MARKERS:
            if w.marks.get(m):
                print(f"  {100.0*w.marks[m]/w.tot:6.2f}%  {m}")
        print(f"  --- callers of EntityLookup.getEntities(Class,...) ---")
        for k, v in w.callers.most_common(5):
            print(f"  {100.0*v/max(1,sum(w.callers.values())):5.1f}%  {' <- '.join(k)}")


if __name__ == "__main__":
    main()
