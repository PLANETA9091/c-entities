# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 74.939 | 139.217 | 201.030 | 308.862 | 13.3 | 1160 |
| mobs-25k | 25000 | 0 | 511.538 | 587.607 | 693.592 | 931.178 | 2.0 | 1834 |
| mixed-150k | 150000 | 0 | 495.696 | 716.126 | 999.575 | 1179.233 | 2.0 | 3195 |
| items-100k-mod | 100000 | 0 | 30.864 | 56.295 | 126.073 | 217.255 | 20.0 | 1724 |
| mixed-150k-mod | 150000 | 0 | 188.421 | 333.223 | 502.161 | 652.890 | 5.3 | 2486 |
| mobs-25k-mod | 25000 | 0 | 2418.130 | 2620.889 | 2799.033 | 2886.670 | 0.4 | 2319 |
| mobs-25k-native-mod | 25000 | 0 | 23.049 | 45.818 | 115.477 | 124.434 | 20.0 | 1739 |
| mobs-25k-native | 25000 | 0 | 490.117 | 560.751 | 636.903 | 853.176 | 2.0 | 1827 |
| spread-100k-mod | 100000 | 0 | 163.572 | 263.700 | 414.798 | 498.042 | 6.1 | 2440 |
| spread-100k | 100000 | 0 | 440.634 | 538.790 | 629.283 | 696.315 | 2.3 | 1972 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

