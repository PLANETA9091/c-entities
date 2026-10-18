#!/usr/bin/env python3
"""Build REPORT.md from a baseline run's results dir."""
import json
import sys
import os


def fmt(x, nd=3):
    return f"{x:.{nd}f}"


def main():
    r = sys.argv[1]
    with open(os.path.join(r, "baseline.json"), encoding="utf-8") as f:
        d = json.load(f)
    ms, sp = d["mspt"], d["spawned"]
    ents = sp["items"] + sp["stands"] + sp["zombies"]
    tiles = sp["hoppers"] + sp["furnaces"]
    tps = min(20.0, 1000.0 / ms["p50"]) if ms["p50"] > 0 else 20.0
    shrink = ""
    if d["samples"]:
        e0, e1 = d["samples"][0]["entities"], d["samples"][-1]["entities"]
        if e0 > 0:
            shrink = f" (merge shrink: {e0} -> {e1})"

    lines = []
    lines.append(f"# Baseline run: `{d['run']}`\n")
    lines.append(f"Server: `{d['server']}`\n")
    lines.append("## Results\n")
    lines.append("| metric | value |")
    lines.append("|---|---|")
    lines.append(f"| entities spawned | {ents} (items {sp['items']} / stands {sp['stands']} / zombies {sp['zombies']}){shrink} |")
    lines.append(f"| block entities | {tiles} (hoppers {sp['hoppers']} / furnaces {sp['furnaces']}) |")
    lines.append(f"| MSPT p50 / p90 / p99 / max, ms | {fmt(ms['p50'])} / {fmt(ms['p90'])} / {fmt(ms['p99'])} / {fmt(ms['max'])} |")
    lines.append(f"| MSPT mean, ms | {fmt(ms['mean'])} |")
    lines.append(f"| TPS estimate (1000/p50) | {fmt(tps, 1)} |")
    lines.append(f"| heap used / max, MB | {fmt(d['heap_mb']['used'],1)} / {fmt(d['heap_mb']['max'],1)} |")
    lines.append(f"| measured ticks | {ms['n']} |")
    lines.append("")
    hp = os.path.join(r, "hotspots.md")
    if os.path.exists(hp):
        hl = open(hp, encoding="utf-8").read().splitlines()
        rows = [l for l in hl if l.startswith("| ") and "---" not in l]
        lines.append("## What burns the time (JFR, top-15 self-time)\n")
        lines.append("| # | % | method |")
        lines.append("|---:|---:|---|")
        for l in rows[1:16]:
            lines.append(l)
        lines.append("")

    with open(os.path.join(r, "REPORT.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    print(f"report: {os.path.join(r, 'REPORT.md')}")


if __name__ == "__main__":
    main()
