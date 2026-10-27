# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 57.546 | 69.278 | 126.068 | 167.667 | 17.4 | 2197 |
| mobs-25k | 25000 | 0 | 360.657 | 411.839 | 472.342 | 521.999 | 2.8 | 1514 |
| mixed-150k | 150000 | 0 | 500.375 | 611.059 | 840.615 | 960.789 | 2.0 | 3126 |
| items-100k-mod | 100000 | 0 | 11.316 | 16.045 | 71.425 | 98.438 | 20.0 | 1356 |
| mixed-150k-mod | 150000 | 0 | 286.793 | 380.540 | 535.164 | 596.383 | 3.5 | 1576 |
| mobs-25k-mod | 25000 | 0 | 186.258 | 330.957 | 572.753 | 694.601 | 5.4 | 1197 |
| spread-100k-mod | 100000 | 0 | 159.202 | 275.326 | 412.546 | 535.655 | 6.3 | 2501 |
| spread-100k | 100000 | 0 | 418.177 | 474.333 | 715.079 | 789.642 | 2.4 | 2130 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

