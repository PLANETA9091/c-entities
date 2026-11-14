# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 60.032 | 102.413 | 150.373 | 188.113 | 16.7 | 775 |
| mobs-25k | 25000 | 0 | 571.856 | 694.384 | 789.980 | 918.242 | 1.7 | 1236 |
| mixed-150k | 150000 | 0 | 369.391 | 509.179 | 698.227 | 916.286 | 2.7 | 2510 |
| items-100k-mod | 100000 | 0 | 22.476 | 38.673 | 100.293 | 193.592 | 20.0 | 1687 |
| mixed-150k-mod | 150000 | 0 | 170.047 | 289.442 | 465.263 | 761.206 | 5.9 | 2509 |
| mobs-25k-mod | 25000 | 0 | 2587.222 | 2775.151 | 3006.533 | 3136.396 | 0.4 | 1564 |
| mobs-25k-native-mod | 25000 | 0 | 26.393 | 45.262 | 123.617 | 139.434 | 20.0 | 1796 |
| mobs-25k-native | 25000 | 0 | 404.599 | 475.195 | 575.961 | 826.365 | 2.5 | 954 |
| spread-100k-mod | 100000 | 0 | 59.564 | 113.973 | 203.203 | 245.452 | 16.8 | 2541 |
| spread-100k | 100000 | 0 | 411.399 | 498.583 | 611.279 | 673.613 | 2.4 | 1991 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

