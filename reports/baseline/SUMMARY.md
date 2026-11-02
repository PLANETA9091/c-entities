# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.329 | 81.319 | 146.652 | 209.985 | 15.5 | 1116 |
| mobs-25k | 25000 | 0 | 521.889 | 578.240 | 630.025 | 656.104 | 1.9 | 1552 |
| mixed-150k | 150000 | 0 | 679.149 | 846.407 | 1170.765 | 1395.584 | 1.5 | 1919 |
| items-100k-mod | 100000 | 0 | 29.881 | 85.823 | 138.639 | 261.874 | 20.0 | 1192 |
| mixed-150k-mod | 150000 | 0 | 286.926 | 395.075 | 600.031 | 779.478 | 3.5 | 1844 |
| mobs-25k-mod | 25000 | 0 | 292.121 | 481.283 | 640.700 | 687.298 | 3.4 | 1412 |
| spread-100k-mod | 100000 | 0 | 268.288 | 388.067 | 503.240 | 613.025 | 3.7 | 2689 |
| spread-100k | 100000 | 0 | 446.961 | 521.817 | 745.327 | 817.312 | 2.2 | 2237 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

