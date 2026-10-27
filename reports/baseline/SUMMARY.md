# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 61.202 | 75.404 | 143.212 | 196.845 | 16.3 | 1976 |
| mobs-25k | 25000 | 0 | 372.869 | 417.797 | 454.965 | 477.779 | 2.7 | 1291 |
| mixed-150k | 150000 | 0 | 437.295 | 534.249 | 743.157 | 833.166 | 2.3 | 2803 |
| items-100k-mod | 100000 | 0 | 10.013 | 13.341 | 73.560 | 107.014 | 20.0 | 1237 |
| mixed-150k-mod | 150000 | 0 | 238.780 | 287.089 | 335.413 | 363.397 | 4.2 | 1989 |
| mobs-25k-mod | 25000 | 0 | 168.577 | 256.690 | 336.559 | 388.980 | 5.9 | 1564 |
| spread-100k-mod | 100000 | 0 | 177.195 | 258.893 | 355.903 | 497.514 | 5.6 | 2047 |
| spread-100k | 100000 | 0 | 434.689 | 497.239 | 735.127 | 838.629 | 2.3 | 2076 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

