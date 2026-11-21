# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 75.437 | 127.659 | 201.466 | 260.753 | 13.3 | 1219 |
| mobs-25k | 25000 | 0 | 374.201 | 446.051 | 546.166 | 637.350 | 2.7 | 1038 |
| mixed-150k | 150000 | 0 | 587.549 | 824.498 | 1175.254 | 1362.497 | 1.7 | 1741 |
| items-100k-mod | 100000 | 0 | 17.259 | 27.202 | 101.424 | 242.537 | 20.0 | 1333 |
| mixed-150k-mod | 150000 | 0 | 61.288 | 102.135 | 254.777 | 388.896 | 16.3 | 1987 |
| mobs-25k-mod | 25000 | 0 | 2296.926 | 2393.967 | 2560.731 | 2607.118 | 0.4 | 1612 |
| mobs-25k-native-mod | 25000 | 0 | 23.096 | 45.277 | 121.134 | 198.169 | 20.0 | 1232 |
| mobs-25k-native | 25000 | 0 | 493.537 | 565.978 | 708.375 | 871.182 | 2.0 | 1273 |
| spread-100k-mod | 100000 | 0 | 49.399 | 89.632 | 198.602 | 345.540 | 20.0 | 1966 |
| spread-100k | 100000 | 0 | 427.225 | 514.018 | 628.481 | 730.037 | 2.3 | 2414 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

