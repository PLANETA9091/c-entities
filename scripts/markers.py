#!/usr/bin/env python3
"""Quantify named method markers in one or two JFRs (share of samples)."""
import subprocess
import sys

MARKERS = [
    "mergeWithNeighbours",
    "getEntitiesOfClass",
    "EntityLookup.getEntities(Class",
    "inactiveTick",
    "checkIfActive",
    "activateEntity",
    "ItemEntity.tick(",
    "Entity.tick(",
    "ServerEntity.sendChanges",
    "updatePlayer",
    "moonrise$tick",
    "itemSkip",
    "EntityTickList.forEach",
    "InsideBlockEffectApplier",
    "setOldPos",
    "getMaxStackSize",
]


def pct(jfr, needle):
    proc = subprocess.run(
        ["jfr", "print", "--events", "jdk.ExecutionSample", jfr],
        capture_output=True, text=True)
    total = 0
    hit = 0
    cur = []
    in_frames = False

    def has():
        return any(needle in f for f in cur)

    for line in proc.stdout.splitlines():
        s = line.strip()
        if s.startswith("jdk.ExecutionSample"):
            if cur:
                total += 1
                if has():
                    hit += 1
            cur = []
            in_frames = False
        elif s == "stackTrace = [":
            in_frames = True
        elif s == "]":
            in_frames = False
        elif in_frames and s and s != "...":
            cur.append(s)
    if cur:
        total += 1
        if has():
            hit += 1
    return total, hit


def main():
    jfrs = sys.argv[1:]
    print(f"{'marker':<38}" + "".join(f"{j.split('/')[-3]:>14}" for j in jfrs))
    for m in MARKERS:
        row = f"{m:<38}"
        for j in jfrs:
            t, h = pct(j, m)
            row += f"{100.0*h/t:>13.2f}%"
        print(row)


if __name__ == "__main__":
    main()
