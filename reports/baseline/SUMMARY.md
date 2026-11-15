# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.091 | 105.383 | 161.108 | 236.484 | 15.4 | 1718 |
| mobs-25k | 25000 | 0 | 505.099 | 577.915 | 681.882 | 732.866 | 2.0 | 1358 |
| mixed-150k | 150000 | 0 | 393.523 | 519.756 | 737.654 | 1015.893 | 2.5 | 2455 |
| items-100k-mod | 100000 | 0 | 27.068 | 44.634 | 110.153 | 210.669 | 20.0 | 1673 |
| mixed-150k-mod | 150000 | 0 | 77.110 | 113.658 | 235.440 | 284.984 | 13.0 | 2156 |
| mobs-25k-mod | 25000 | 0 | 2061.587 | 2144.171 | 2293.279 | 2390.674 | 0.5 | 2230 |
| mobs-25k-native-mod | 25000 | 0 | 23.110 | 48.844 | 120.036 | 219.768 | 20.0 | 1663 |
| mobs-25k-native | 25000 | 0 | 300.509 | 375.341 | 467.164 | 524.743 | 3.3 | 1237 |
| spread-100k-mod | 100000 | 0 | 58.531 | 108.827 | 205.122 | 341.936 | 17.1 | 2169 |
| spread-100k | 100000 | 0 | 322.140 | 404.655 | 501.100 | 823.052 | 3.1 | 1854 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

