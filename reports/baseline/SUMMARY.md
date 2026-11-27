# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 81.835 | 143.769 | 211.181 | 284.299 | 12.2 | 2658 |
| mobs-25k | 25000 | 0 | 597.279 | 671.746 | 764.186 | 925.388 | 1.7 | 1779 |
| mixed-150k | 150000 | 0 | 562.434 | 772.763 | 1025.698 | 1156.005 | 1.8 | 1862 |
| items-100k-mod | 100000 | 0 | 16.121 | 34.745 | 117.160 | 220.072 | 20.0 | 1346 |
| mixed-150k-mod | 150000 | 0 | 62.860 | 114.857 | 270.378 | 345.633 | 15.9 | 2043 |
| mobs-25k-mod | 25000 | 0 | 3301.826 | 3433.359 | 3617.634 | 3725.764 | 0.3 | 1521 |
| mobs-25k-native-mod | 25000 | 0 | 20.059 | 43.322 | 117.904 | 140.156 | 20.0 | 1335 |
| mobs-25k-native | 25000 | 0 | 508.014 | 575.132 | 668.073 | 767.349 | 2.0 | 1480 |
| spread-100k-mod | 100000 | 0 | 44.618 | 67.557 | 186.959 | 248.305 | 20.0 | 2127 |
| spread-100k | 100000 | 0 | 357.324 | 448.840 | 540.636 | 641.087 | 2.8 | 1852 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

