# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.463 | 113.023 | 180.151 | 248.629 | 14.0 | 1873 |
| mobs-25k | 25000 | 0 | 526.787 | 594.737 | 690.368 | 953.447 | 1.9 | 990 |
| mixed-150k | 150000 | 0 | 504.803 | 653.818 | 874.346 | 982.641 | 2.0 | 3131 |
| items-100k-mod | 100000 | 0 | 34.052 | 56.835 | 121.366 | 227.728 | 20.0 | 1612 |
| mixed-150k-mod | 150000 | 0 | 88.256 | 140.572 | 263.107 | 367.585 | 11.3 | 2050 |
| mobs-25k-mod | 25000 | 0 | 2517.622 | 2725.235 | 2967.684 | 3155.172 | 0.4 | 2384 |
| mobs-25k-native-mod | 25000 | 0 | 18.895 | 34.736 | 108.749 | 235.850 | 20.0 | 1554 |
| mobs-25k-native | 25000 | 0 | 277.633 | 341.288 | 413.852 | 555.325 | 3.6 | 1516 |
| spread-100k-mod | 100000 | 0 | 66.640 | 130.925 | 237.030 | 377.112 | 15.0 | 2146 |
| spread-100k | 100000 | 0 | 548.716 | 653.010 | 747.218 | 920.769 | 1.8 | 2356 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

