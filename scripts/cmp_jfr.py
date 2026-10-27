#!/usr/bin/env python3
"""Compare stack composition of two JFR recordings.

Usage: cmp_jfr.py <a.jfr> <b.jfr>

For each recording counts execution samples whose stack contains each
marker frame (merge scan, vanilla entity lookup, woven bridge, item tick
guard, tracker work) and prints an aligned table. Sample = CPU time proxy.
"""
import subprocess
import sys
import re
import os

JFR_BIN = os.environ.get("JFR_BIN", "jfr")
from collections import Counter

MARKERS = {
    "merge scan (getEntities(Class...))": "ChunkEntitySlices.getEntities(Class",
    "vanilla AABB.intersects": "AABB.intersects(",
    "CeHooks.getEntities (woven)": "CeHooks.getEntities",
    "CeHooks.fastQuery": "CeHooks.fastQuery",
    "CeHooks.rowQuery": "CeHooks.rowQuery",
    "CeHooks.mirror": "CeHooks.mirror(",
    "item tick (ItemEntity.tick)": "ItemEntity.tick(",
    "rest guard (hookSkipGuard)": "hookSkipGuard",
    "mergeWithNeighbours": "mergeWithNeighbours",
    "tracker (updatePlayer)": "TrackedEntity.updatePlayer",
    "CeBaseline plugin": "cebaseline",
    "entity tick loop": "ServerLevel.tick(",
}


def stacks(jfr: str):
    proc = subprocess.run(
        [JFR_BIN, "print", "--events", "jdk.ExecutionSample", jfr],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr[:400])
    total = 0
    hits = Counter()
    cur = []
    for line in proc.stdout.splitlines():
        if line.strip().startswith("jdk.ExecutionSample"):
            if cur:
                total += 1
                text = "\n".join(cur)
                for name, pat in MARKERS.items():
                    if pat in text:
                        hits[name] += 1
            cur = []
        else:
            cur.append(line)
    if cur:
        total += 1
        text = "\n".join(cur)
        for name, pat in MARKERS.items():
            if pat in text:
                hits[name] += 1
    return total, hits


def main():
    a, b = sys.argv[1], sys.argv[2]
    ta, ha = stacks(a)
    tb, hb = stacks(b)
    print(f"{'marker':42s} {'A %':>8s} {'B %':>8s} {'A#':>8s} {'B#':>8s}")
    print(f"(A={a.split('/')[-3] if '/' in a else a} total={ta}, B={b.split('/')[-3] if '/' in b else b} total={tb})")
    for name in MARKERS:
        pa = ha.get(name, 0) / ta * 100
        pb = hb.get(name, 0) / tb * 100
        print(f"{name:42s} {pa:7.2f}% {pb:7.2f}% {ha.get(name,0):8d} {hb.get(name,0):8d}")


main()
