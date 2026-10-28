# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.415 | 90.496 | 155.910 | 196.969 | 14.4 | 1886 |
| mobs-25k | 25000 | 0 | 526.335 | 593.245 | 653.866 | 664.269 | 1.9 | 922 |
| mixed-150k | 150000 | 0 | 441.728 | 528.150 | 758.977 | 911.208 | 2.3 | 2552 |
| items-100k-mod | 100000 | 0 | 33.946 | 89.967 | 153.607 | 234.178 | 20.0 | 1316 |
| mixed-150k-mod | 150000 | 0 | 376.426 | 489.589 | 634.084 | 772.566 | 2.7 | 2158 |
| mobs-25k-mod | 25000 | 0 | 204.308 | 413.103 | 624.445 | 677.400 | 4.9 | 1643 |
| spread-100k-mod | 100000 | 0 | 204.244 | 340.335 | 518.747 | 677.099 | 4.9 | 1900 |
| spread-100k | 100000 | 0 | 515.049 | 597.018 | 828.915 | 930.228 | 1.9 | 2104 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

