# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 51.990 | 86.975 | 137.907 | 167.281 | 19.2 | 2097 |
| mobs-25k | 25000 | 0 | 543.203 | 620.243 | 683.857 | 812.244 | 1.8 | 1083 |
| mixed-150k | 150000 | 0 | 481.060 | 715.299 | 866.302 | 1205.005 | 2.1 | 3083 |
| items-100k-mod | 100000 | 0 | 20.376 | 37.140 | 91.471 | 139.407 | 20.0 | 1713 |
| mixed-150k-mod | 150000 | 0 | 91.617 | 158.843 | 277.061 | 447.189 | 10.9 | 2096 |
| mobs-25k-mod | 25000 | 0 | 2476.921 | 2661.273 | 2815.139 | 2926.111 | 0.4 | 2030 |
| mobs-25k-native-mod | 25000 | 0 | 16.191 | 29.826 | 99.237 | 153.755 | 20.0 | 1575 |
| mobs-25k-native | 25000 | 0 | 500.326 | 582.051 | 682.577 | 766.147 | 2.0 | 1669 |
| spread-100k-mod | 100000 | 0 | 65.352 | 124.612 | 228.604 | 331.787 | 15.3 | 2261 |
| spread-100k | 100000 | 0 | 453.571 | 561.039 | 669.119 | 734.899 | 2.2 | 1956 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

