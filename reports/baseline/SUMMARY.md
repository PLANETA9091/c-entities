# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 88.701 | 160.257 | 260.106 | 313.580 | 11.3 | 1392 |
| mobs-25k | 25000 | 0 | 399.809 | 460.850 | 548.403 | 720.092 | 2.5 | 1850 |
| mixed-150k | 150000 | 0 | 563.326 | 756.447 | 984.464 | 1140.488 | 1.8 | 1883 |
| items-100k-mod | 100000 | 0 | 21.918 | 47.825 | 117.462 | 189.365 | 20.0 | 1641 |
| mixed-150k-mod | 150000 | 0 | 71.642 | 131.070 | 280.965 | 400.090 | 14.0 | 2295 |
| mobs-25k-mod | 25000 | 0 | 313.217 | 1612.316 | 2737.118 | 3010.006 | 3.2 | 2489 |
| mobs-25k-native-mod | 25000 | 0 | 26.235 | 63.885 | 163.046 | 326.167 | 20.0 | 1781 |
| mobs-25k-native | 25000 | 0 | 546.249 | 664.480 | 773.804 | 973.385 | 1.8 | 1240 |
| spread-100k-mod | 100000 | 0 | 47.133 | 67.022 | 185.932 | 357.959 | 20.0 | 2527 |
| spread-100k | 100000 | 0 | 408.683 | 496.051 | 612.741 | 705.390 | 2.4 | 1701 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

