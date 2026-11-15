# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.829 | 114.473 | 178.884 | 255.544 | 14.5 | 1096 |
| mobs-25k | 25000 | 0 | 482.038 | 557.925 | 648.751 | 735.713 | 2.1 | 1731 |
| mixed-150k | 150000 | 0 | 475.404 | 659.880 | 896.862 | 1014.034 | 2.1 | 3037 |
| items-100k-mod | 100000 | 0 | 30.012 | 51.268 | 120.504 | 235.527 | 20.0 | 1685 |
| mixed-150k-mod | 150000 | 0 | 87.458 | 136.740 | 275.304 | 426.316 | 11.4 | 2232 |
| mobs-25k-mod | 25000 | 0 | 2100.681 | 2221.691 | 2338.944 | 2423.468 | 0.5 | 1656 |
| mobs-25k-native-mod | 25000 | 0 | 25.342 | 57.817 | 126.083 | 148.924 | 20.0 | 1581 |
| mobs-25k-native | 25000 | 0 | 406.673 | 478.924 | 573.553 | 816.701 | 2.5 | 1275 |
| spread-100k-mod | 100000 | 0 | 58.295 | 96.070 | 206.540 | 241.049 | 17.2 | 2680 |
| spread-100k | 100000 | 0 | 353.349 | 440.939 | 538.334 | 695.654 | 2.8 | 1818 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

