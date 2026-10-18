#!/usr/bin/env python3
"""Merge all baseline runs into one summary markdown table."""
import json
import glob
import os
import sys


def fmt(x, nd=3):
    return f"{x:.{nd}f}".replace(",", ".")


def main():
    root = sys.argv[1] if len(sys.argv) > 1 else "bench"
    rows = []
    for jp in sorted(glob.glob(os.path.join(root, "*", "results", "baseline.json"))):
        with open(jp, encoding="utf-8") as f:
            d = json.load(f)
        sp = d["spawned"]
        ents = sp["items"] + sp["stands"] + sp["zombies"]
        tiles = sp["hoppers"] + sp["furnaces"]
        ms = d["mspt"]
        rows.append((d["run"], ents, tiles, ms, d["heap_mb"]["used"]))

    order = ["control", "items-25k", "items-100k", "mobs-10k", "mobs-25k",
             "mixed-50k", "mixed-150k", "tiles-25k", "tiles-100k"]
    rows.sort(key=lambda r: (order.index(r[0]) if r[0] in order else 99))

    print("# Базовый профиль Paper 1.21.8 (без оптимизаций)\n")
    print("Снято на GitHub Actions (ubuntu-latest, 4 vCPU), JFR включён (~2-5% оверхед одинаково для всех прогонов).\n")
    print("| прогон | энтити | тайл-энтити | MSPT p50 | p90 | p99 | max, мс | TPS-оценка | heap, МБ |")
    print("|---|---:|---:|---:|---:|---:|---:|---:|---:|")
    for run, ents, tiles, ms, heap in rows:
        tps = min(20.0, 1000.0 / ms["p50"]) if ms["p50"] > 0 else 20.0
        print(f"| {run} | {ents} | {tiles} | {fmt(ms['p50'])} | {fmt(ms['p90'])} | "
              f"{fmt(ms['p99'])} | {fmt(ms['max'])} | {fmt(tps,1)} | {fmt(heap,0)} |")
    print("\nДетальные отчёты: `reports/baseline/<прогон>/REPORT.md`; сырые данные: `baseline.json` в артефактах прогона.\n")


if __name__ == "__main__":
    main()
