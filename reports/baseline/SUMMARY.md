# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 59.839 | 72.604 | 132.030 | 193.093 | 16.7 | 1248 |
| mobs-25k | 25000 | 0 | 388.750 | 454.069 | 524.839 | 617.075 | 2.6 | 1651 |
| mixed-150k | 150000 | 0 | 467.263 | 567.007 | 767.118 | 867.653 | 2.1 | 2966 |
| items-100k-mod | 100000 | 0 | 31.145 | 46.469 | 122.014 | 167.456 | 20.0 | 1253 |
| mixed-150k-mod | 150000 | 0 | 105.349 | 218.806 | 373.456 | 583.204 | 9.5 | 1361 |
| mobs-25k-mod | 25000 | 0 | 0.331 | 0.538 | 1.528 | 4.249 | 20.0 | 1788 |
| spread-100k-mod | 100000 | 0 | 87.693 | 139.829 | 281.560 | 474.704 | 11.4 | 1930 |
| spread-100k | 100000 | 0 | 325.125 | 383.346 | 578.197 | 644.017 | 3.1 | 2659 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

