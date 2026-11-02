# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.344 | 94.954 | 173.067 | 206.266 | 14.0 | 886 |
| mobs-25k | 25000 | 0 | 518.698 | 571.993 | 627.043 | 711.507 | 1.9 | 1614 |
| mixed-150k | 150000 | 0 | 669.705 | 817.923 | 1187.246 | 1333.072 | 1.5 | 2992 |
| items-100k-mod | 100000 | 0 | 43.974 | 105.583 | 195.484 | 250.682 | 20.0 | 1261 |
| mixed-150k-mod | 150000 | 0 | 292.568 | 411.706 | 610.188 | 729.376 | 3.4 | 1643 |
| mobs-25k-mod | 25000 | 0 | 309.733 | 561.641 | 690.990 | 807.728 | 3.2 | 1484 |
| spread-100k-mod | 100000 | 0 | 275.885 | 438.221 | 557.542 | 767.507 | 3.6 | 2610 |
| spread-100k | 100000 | 0 | 437.215 | 492.368 | 706.768 | 801.983 | 2.3 | 1926 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

