# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 78.844 | 140.366 | 208.567 | 319.247 | 12.7 | 1447 |
| mobs-25k | 25000 | 0 | 564.769 | 640.620 | 745.826 | 879.632 | 1.8 | 2087 |
| mixed-150k | 150000 | 0 | 614.043 | 862.271 | 1240.314 | 1413.662 | 1.6 | 2411 |
| items-100k-mod | 100000 | 0 | 24.091 | 45.344 | 126.812 | 298.443 | 20.0 | 1336 |
| mixed-150k-mod | 150000 | 0 | 70.895 | 113.508 | 290.226 | 411.800 | 14.1 | 1979 |
| mobs-25k-mod | 25000 | 0 | 3065.146 | 3211.404 | 3408.083 | 3501.851 | 0.3 | 1308 |
| mobs-25k-native-mod | 25000 | 0 | 15.442 | 26.968 | 88.350 | 96.603 | 20.0 | 1148 |
| mobs-25k-native | 25000 | 0 | 506.099 | 578.379 | 689.257 | 790.561 | 2.0 | 1522 |
| spread-100k-mod | 100000 | 0 | 38.618 | 58.139 | 166.754 | 227.422 | 20.0 | 2150 |
| spread-100k | 100000 | 0 | 375.970 | 470.838 | 566.642 | 631.882 | 2.7 | 2256 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

