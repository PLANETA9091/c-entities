# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.873 | 79.083 | 140.809 | 181.632 | 15.7 | 1237 |
| mobs-25k | 25000 | 0 | 314.380 | 361.466 | 406.084 | 425.963 | 3.2 | 1161 |
| mixed-150k | 150000 | 0 | 542.477 | 648.663 | 878.815 | 998.477 | 1.8 | 2733 |
| items-100k-mod | 100000 | 0 | 34.811 | 85.073 | 139.553 | 232.131 | 20.0 | 1387 |
| mixed-150k-mod | 150000 | 0 | 248.738 | 353.415 | 501.479 | 623.620 | 4.0 | 1868 |
| mobs-25k-mod | 25000 | 0 | 239.505 | 375.140 | 481.561 | 570.480 | 4.2 | 1724 |
| spread-100k-mod | 100000 | 0 | 332.014 | 506.559 | 668.424 | 914.941 | 3.0 | 2018 |
| spread-100k | 100000 | 0 | 539.743 | 596.717 | 862.524 | 911.275 | 1.9 | 2113 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

