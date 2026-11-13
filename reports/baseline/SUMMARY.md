# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.960 | 121.385 | 180.553 | 220.588 | 14.3 | 1531 |
| mobs-25k | 25000 | 0 | 551.376 | 621.157 | 686.925 | 953.869 | 1.8 | 1474 |
| mixed-150k | 150000 | 0 | 460.467 | 639.644 | 829.643 | 956.163 | 2.2 | 3082 |
| items-100k-mod | 100000 | 0 | 24.247 | 37.050 | 100.944 | 245.851 | 20.0 | 1644 |
| mixed-150k-mod | 150000 | 0 | 163.090 | 277.871 | 397.003 | 489.220 | 6.1 | 2151 |
| mobs-25k-mod | 25000 | 0 | 3308.742 | 3532.545 | 3745.549 | 3957.857 | 0.3 | 1645 |
| mobs-25k-native-mod | 25000 | 0 | 46.818 | 72.783 | 217.276 | 385.902 | 20.0 | 1547 |
| mobs-25k-native | 25000 | 0 | 495.680 | 573.345 | 682.247 | 973.953 | 2.0 | 1717 |
| spread-100k-mod | 100000 | 0 | 179.878 | 253.167 | 398.916 | 450.522 | 5.6 | 1978 |
| spread-100k | 100000 | 0 | 451.849 | 544.776 | 636.668 | 696.936 | 2.2 | 2007 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

