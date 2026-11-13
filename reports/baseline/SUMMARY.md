# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.022 | 111.341 | 164.843 | 189.711 | 14.7 | 1763 |
| mobs-25k | 25000 | 0 | 496.725 | 564.859 | 681.875 | 904.817 | 2.0 | 1254 |
| mixed-150k | 150000 | 0 | 566.252 | 774.746 | 1044.811 | 1271.769 | 1.8 | 1924 |
| items-100k-mod | 100000 | 0 | 22.605 | 33.842 | 102.576 | 174.589 | 20.0 | 1744 |
| mixed-150k-mod | 150000 | 0 | 170.864 | 297.067 | 439.816 | 545.584 | 5.9 | 2219 |
| mobs-25k-mod | 25000 | 0 | 279.265 | 512.780 | 775.799 | 1040.240 | 3.6 | 2080 |
| mobs-25k-native-mod | 25000 | 0 | 45.279 | 93.632 | 214.873 | 252.820 | 20.0 | 1498 |
| mobs-25k-native | 25000 | 0 | 557.473 | 632.776 | 744.352 | 878.979 | 1.8 | 1862 |
| spread-100k-mod | 100000 | 0 | 164.175 | 220.337 | 336.266 | 385.703 | 6.1 | 2353 |
| spread-100k | 100000 | 0 | 353.873 | 440.034 | 538.352 | 642.859 | 2.8 | 1770 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

