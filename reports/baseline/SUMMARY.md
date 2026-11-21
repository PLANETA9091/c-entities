# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 75.128 | 121.256 | 181.296 | 803.678 | 13.3 | 1770 |
| mobs-25k | 25000 | 0 | 497.325 | 567.421 | 644.347 | 673.628 | 2.0 | 1339 |
| mixed-150k | 150000 | 0 | 488.139 | 699.400 | 938.342 | 1077.460 | 2.0 | 2882 |
| items-100k-mod | 100000 | 0 | 16.590 | 30.370 | 105.424 | 178.989 | 20.0 | 1261 |
| mixed-150k-mod | 150000 | 0 | 59.116 | 102.461 | 220.538 | 316.929 | 16.9 | 1786 |
| mobs-25k-mod | 25000 | 0 | 2550.467 | 2657.322 | 2813.949 | 2913.605 | 0.4 | 1888 |
| mobs-25k-native-mod | 25000 | 0 | 20.806 | 41.422 | 114.601 | 163.391 | 20.0 | 1387 |
| mobs-25k-native | 25000 | 0 | 407.010 | 479.533 | 594.320 | 688.892 | 2.5 | 1317 |
| spread-100k-mod | 100000 | 0 | 40.174 | 61.249 | 158.169 | 202.748 | 20.0 | 2013 |
| spread-100k | 100000 | 0 | 305.832 | 389.155 | 473.819 | 558.084 | 3.3 | 3083 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

