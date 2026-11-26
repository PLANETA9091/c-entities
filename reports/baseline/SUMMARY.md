# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.521 | 120.183 | 174.678 | 246.515 | 14.2 | 895 |
| mobs-25k | 25000 | 0 | 512.744 | 580.937 | 672.614 | 792.006 | 2.0 | 1616 |
| mixed-150k | 150000 | 0 | 489.908 | 663.921 | 880.335 | 986.110 | 2.0 | 3288 |
| items-100k-mod | 100000 | 0 | 14.223 | 25.396 | 99.720 | 2285.328 | 20.0 | 1572 |
| mixed-150k-mod | 150000 | 0 | 57.444 | 96.025 | 232.829 | 321.219 | 17.4 | 1903 |
| mobs-25k-mod | 25000 | 0 | 3139.410 | 3337.094 | 3540.069 | 3629.865 | 0.3 | 1699 |
| mobs-25k-native-mod | 25000 | 0 | 20.875 | 43.886 | 110.222 | 190.063 | 20.0 | 1251 |
| mobs-25k-native | 25000 | 0 | 409.400 | 475.660 | 594.855 | 706.262 | 2.4 | 1459 |
| spread-100k-mod | 100000 | 0 | 47.311 | 69.924 | 200.580 | 345.246 | 20.0 | 2066 |
| spread-100k | 100000 | 0 | 431.233 | 523.556 | 625.167 | 755.293 | 2.3 | 2016 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

