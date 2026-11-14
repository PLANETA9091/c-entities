# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.159 | 116.691 | 171.195 | 209.235 | 14.3 | 1196 |
| mobs-25k | 25000 | 0 | 468.080 | 548.381 | 683.422 | 719.200 | 2.1 | 1021 |
| mixed-150k | 150000 | 0 | 614.038 | 791.214 | 1183.574 | 1465.332 | 1.6 | 1750 |
| items-100k-mod | 100000 | 0 | 29.721 | 47.796 | 117.201 | 227.319 | 20.0 | 1478 |
| mixed-150k-mod | 150000 | 0 | 182.705 | 351.716 | 527.205 | 614.144 | 5.5 | 2450 |
| mobs-25k-mod | 25000 | 0 | 2588.191 | 2767.922 | 2895.871 | 3101.180 | 0.4 | 2220 |
| mobs-25k-native-mod | 25000 | 0 | 22.292 | 50.651 | 117.631 | 199.238 | 20.0 | 1508 |
| mobs-25k-native | 25000 | 0 | 385.725 | 455.763 | 552.869 | 639.821 | 2.6 | 1066 |
| spread-100k-mod | 100000 | 0 | 159.733 | 232.980 | 366.147 | 538.464 | 6.3 | 3202 |
| spread-100k | 100000 | 0 | 366.871 | 445.839 | 554.673 | 639.758 | 2.7 | 1895 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

