# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.648 | 105.809 | 161.716 | 228.657 | 15.2 | 1057 |
| mobs-25k | 25000 | 0 | 745.858 | 817.801 | 903.952 | 986.122 | 1.3 | 1422 |
| mixed-150k | 150000 | 0 | 561.969 | 800.982 | 1108.172 | 1265.137 | 1.8 | 2368 |
| items-100k-mod | 100000 | 0 | 13.049 | 22.802 | 95.480 | 229.196 | 20.0 | 1273 |
| mixed-150k-mod | 150000 | 0 | 59.992 | 102.430 | 236.630 | 416.870 | 16.7 | 1815 |
| mobs-25k-mod | 25000 | 0 | 3617.463 | 3857.271 | 4147.930 | 4433.799 | 0.3 | 1405 |
| mobs-25k-native-mod | 25000 | 0 | 17.813 | 33.171 | 107.482 | 144.369 | 20.0 | 1422 |
| mobs-25k-native | 25000 | 0 | 332.062 | 402.095 | 540.228 | 1000.314 | 3.0 | 1648 |
| spread-100k-mod | 100000 | 0 | 33.436 | 55.712 | 151.095 | 187.401 | 20.0 | 1953 |
| spread-100k | 100000 | 0 | 393.974 | 496.101 | 603.533 | 657.681 | 2.5 | 1932 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

