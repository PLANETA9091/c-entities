#!/usr/bin/env python3
"""Poor-man's profiler: aggregate JDK Flight Recording into top hot frames.

Usage: jfr_top.py <rec.jfr>  >  hotspots.md
Counts leaf frames (self-time proxy) and depth-1 callers from
jdk.ExecutionSample. Also sums jdk.GCPause durations.
"""
import subprocess
import sys
import re
import os

JFR_BIN = os.environ.get("JFR_BIN", "jfr")
from collections import Counter


def samples(jfr: str):
    proc = subprocess.run(
        [JFR_BIN, "print", "--events", "jdk.ExecutionSample", jfr],
        capture_output=True, text=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr[:500])
    leaf = Counter()
    caller = Counter()
    total = 0
    in_frames = False
    frames = []
    for line in proc.stdout.splitlines():
        s = line.strip()
        if s == "stackTrace = [":
            in_frames = True
            frames = []
            continue
        if in_frames:
            if s == "]":
                in_frames = False
                if frames:
                    total += 1
                    leaf[frames[0]] += 1
                    if len(frames) > 1:
                        caller[frames[1]] += 1
            elif s and not s.startswith("startTime") and not s.startswith("objectClass"):
                # strip thread-state-ish noise, keep "class.method(File:line)"
                if re.match(r"^[\w$.]+\.[\w$<>]+\(", s):
                    frames.append(s)
    return leaf, caller, total


def gc(jfr: str):
    proc = subprocess.run([JFR_BIN, "print", "--events", "jdk.GCPause", jfr],
                          capture_output=True, text=True)
    total_ms = 0.0
    count = 0
    for m in re.finditer(r"duration = ([\d.]+) ms", proc.stdout):
        total_ms += float(m.group(1))
        count += 1
    return count, total_ms


def main():
    jfr = sys.argv[1]
    leaf, caller, total = samples(jfr)
    print("# JVM hotspots (JFR ExecutionSample)\n")
    if total == 0:
        print("(no samples — recording empty?)\n")
        return
    print(f"Samples: {total}\n")
    print("## Top 60 by self-time (leaf frames)\n")
    print("| # | % | method |")
    print("|---:|---:|---|")
    for i, (fr, n) in enumerate(leaf.most_common(60)):
        print(f"| {i+1} | {100.0*n/total:.2f}% | `{fr}` |")
    print("\n## Top 30 callers of hot leaves\n")
    print("| # | % | method |")
    print("|---:|---:|---|")
    for i, (fr, n) in enumerate(caller.most_common(30)):
        print(f"| {i+1} | {100.0*n/total:.2f}% | `{fr}` |")
    try:
        count, ms = gc(jfr)
        print(f"\nGC pauses: {count}, total {ms:.0f} ms\n")
    except Exception:
        pass


if __name__ == "__main__":
    main()
