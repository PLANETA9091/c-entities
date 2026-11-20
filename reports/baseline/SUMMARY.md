# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.126 | 124.599 | 187.684 | 249.861 | 14.3 | 2337 |
| mobs-25k | 25000 | 0 | 570.822 | 642.041 | 735.530 | 868.804 | 1.8 | 1738 |
| mixed-150k | 150000 | 0 | 481.537 | 657.363 | 853.121 | 1031.451 | 2.1 | 2852 |
| items-100k-mod | 100000 | 0 | 36.583 | 100.461 | 164.459 | 202.441 | 20.0 | 1492 |
| mixed-150k-mod | 150000 | 0 | 109.679 | 236.949 | 405.119 | 755.134 | 9.1 | 2285 |
| mobs-25k-mod | 25000 | 0 | 3108.132 | 3223.947 | 3397.660 | 3548.987 | 0.3 | 1853 |
| mobs-25k-native-mod | 25000 | 0 | 24.919 | 65.378 | 125.042 | 236.328 | 20.0 | 1611 |
| mobs-25k-native | 25000 | 0 | 322.495 | 385.171 | 487.526 | 567.139 | 3.1 | 1760 |
| spread-100k-mod | 100000 | 0 | 76.852 | 133.347 | 251.055 | 317.037 | 13.0 | 1976 |
| spread-100k | 100000 | 0 | 422.570 | 529.442 | 651.117 | 710.331 | 2.4 | 1736 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

