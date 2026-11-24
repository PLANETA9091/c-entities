# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 74.396 | 120.033 | 179.211 | 214.049 | 13.4 | 1159 |
| mobs-25k | 25000 | 0 | 339.423 | 407.069 | 489.550 | 528.409 | 2.9 | 1186 |
| mixed-150k | 150000 | 0 | 408.723 | 560.611 | 777.544 | 906.842 | 2.4 | 2682 |
| items-100k-mod | 100000 | 0 | 15.115 | 29.161 | 115.379 | 233.901 | 20.0 | 1388 |
| mixed-150k-mod | 150000 | 0 | 55.022 | 84.465 | 226.105 | 332.437 | 18.2 | 2008 |
| mobs-25k-mod | 25000 | 0 | 1359.013 | 2243.030 | 2454.703 | 2751.697 | 0.7 | 1797 |
| mobs-25k-native-mod | 25000 | 0 | 20.698 | 39.222 | 115.845 | 188.624 | 20.0 | 1185 |
| mobs-25k-native | 25000 | 0 | 339.959 | 407.731 | 514.924 | 650.408 | 2.9 | 1366 |
| spread-100k-mod | 100000 | 0 | 44.130 | 64.019 | 194.846 | 376.439 | 20.0 | 2082 |
| spread-100k | 100000 | 0 | 355.437 | 446.362 | 541.223 | 676.568 | 2.8 | 1921 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

