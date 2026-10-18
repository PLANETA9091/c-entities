#!/usr/bin/env python3
"""Merge all baseline runs into one summary markdown table.

Handles both artifact layouts:
  bench/<run>/baseline.json          (per-artifact dir)
  bench/<run>/results/baseline.json  (local run layout)
"""
import json
import glob
import os
import sys


def fmt(x, nd=3):
    return f"{x:.{nd}f}"


def main():
    root = sys.argv[1] if len(sys.argv) > 1 else "bench"
    rows = []
    seen = set()
    for jp in sorted(glob.glob(os.path.join(root, "*", "baseline.json"))
                     + glob.glob(os.path.join(root, "*", "results", "baseline.json"))):
        with open(jp, encoding="utf-8") as f:
            d = json.load(f)
        if d["run"] in seen:
            continue
        seen.add(d["run"])
        sp = d["spawned"]
        ents = sp["items"] + sp["stands"] + sp["zombies"]
        tiles = sp["hoppers"] + sp["furnaces"]
        rows.append((d["run"], ents, tiles, d["mspt"], d["heap_mb"]["used"]))

    order = ["control", "items-25k", "items-100k", "mobs-10k", "mobs-25k",
             "mixed-50k", "mixed-150k", "tiles-25k", "tiles-100k"]
    rows.sort(key=lambda r: (order.index(r[0]) if r[0] in order else 99))

    print("# Baseline profile of Paper 1.21.8 (no optimizations)\n")
    print("Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).\n")
    print("| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |")
    print("|---|---:|---:|---:|---:|---:|---:|---:|---:|")
    for run, ents, tiles, ms, heap in rows:
        tps = min(20.0, 1000.0 / ms["p50"]) if ms["p50"] > 0 else 20.0
        print(f"| {run} | {ents} | {tiles} | {fmt(ms['p50'])} | {fmt(ms['p90'])} | "
              f"{fmt(ms['p99'])} | {fmt(ms['max'])} | {fmt(tps,1)} | {fmt(heap,0)} |")
    print("\nPer-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.\n")


if __name__ == "__main__":
    main()
