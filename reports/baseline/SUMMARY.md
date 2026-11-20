# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.913 | 116.771 | 172.564 | 230.678 | 14.5 | 1397 |
| mobs-25k | 25000 | 0 | 540.245 | 613.709 | 703.296 | 936.836 | 1.9 | 1413 |
| mixed-150k | 150000 | 0 | 433.630 | 592.056 | 864.094 | 923.365 | 2.3 | 3337 |
| items-100k-mod | 100000 | 0 | 14.955 | 28.042 | 96.451 | 165.400 | 20.0 | 1271 |
| mixed-150k-mod | 150000 | 0 | 61.345 | 106.528 | 234.534 | 286.832 | 16.3 | 1752 |
| mobs-25k-mod | 25000 | 0 | 2918.098 | 3083.234 | 3309.730 | 3418.132 | 0.3 | 1907 |
| mobs-25k-native-mod | 25000 | 0 | 19.740 | 40.089 | 109.805 | 220.681 | 20.0 | 1286 |
| mobs-25k-native | 25000 | 0 | 378.366 | 452.273 | 548.095 | 660.675 | 2.6 | 1104 |
| spread-100k-mod | 100000 | 0 | 54.560 | 97.869 | 214.840 | 282.533 | 18.3 | 2155 |
| spread-100k | 100000 | 0 | 560.725 | 716.577 | 860.577 | 970.770 | 1.8 | 2426 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

