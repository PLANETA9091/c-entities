# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.128 | 108.793 | 156.380 | 205.477 | 14.9 | 868 |
| mobs-25k | 25000 | 0 | 391.685 | 466.285 | 575.852 | 717.220 | 2.6 | 1045 |
| mixed-150k | 150000 | 0 | 681.048 | 859.135 | 1284.057 | 1478.380 | 1.5 | 2014 |
| items-100k-mod | 100000 | 0 | 12.057 | 24.181 | 91.730 | 148.422 | 20.0 | 1267 |
| mixed-150k-mod | 150000 | 0 | 62.212 | 104.943 | 255.054 | 407.652 | 16.1 | 1915 |
| mobs-25k-mod | 25000 | 0 | 2335.993 | 2464.637 | 2625.242 | 2746.073 | 0.4 | 1900 |
| mobs-25k-native-mod | 25000 | 0 | 21.911 | 41.517 | 115.130 | 130.657 | 20.0 | 1271 |
| mobs-25k-native | 25000 | 0 | 533.181 | 604.163 | 730.596 | 930.431 | 1.9 | 927 |
| spread-100k-mod | 100000 | 0 | 42.734 | 69.036 | 161.949 | 473.570 | 20.0 | 2144 |
| spread-100k | 100000 | 0 | 329.720 | 410.630 | 511.584 | 594.573 | 3.0 | 2458 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

