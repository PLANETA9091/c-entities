# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.893 | 78.760 | 144.105 | 191.272 | 15.7 | 1177 |
| mobs-25k | 25000 | 0 | 368.889 | 411.601 | 453.460 | 483.144 | 2.7 | 1755 |
| mixed-150k | 150000 | 0 | 550.914 | 756.599 | 1107.696 | 1494.784 | 1.8 | 1587 |
| items-100k-mod | 100000 | 0 | 33.034 | 50.910 | 166.914 | 214.108 | 20.0 | 1480 |
| mixed-150k-mod | 150000 | 0 | 463.862 | 510.632 | 567.209 | 608.343 | 2.2 | 2136 |
| mobs-25k-mod | 25000 | 0 | 194.554 | 226.363 | 264.385 | 295.406 | 5.1 | 1562 |
| spread-100k-mod | 100000 | 0 | 257.439 | 310.294 | 433.689 | 464.401 | 3.9 | 1948 |
| spread-100k | 100000 | 0 | 396.416 | 451.021 | 641.599 | 702.071 | 2.5 | 2016 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

