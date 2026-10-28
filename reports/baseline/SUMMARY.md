# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.627 | 89.421 | 156.626 | 221.111 | 14.2 | 1408 |
| mobs-25k | 25000 | 0 | 433.395 | 489.495 | 544.837 | 587.550 | 2.3 | 1285 |
| mixed-150k | 150000 | 0 | 663.913 | 822.353 | 1134.976 | 1272.461 | 1.5 | 3238 |
| items-100k-mod | 100000 | 0 | 30.049 | 75.183 | 147.345 | 213.691 | 20.0 | 1298 |
| mixed-150k-mod | 150000 | 0 | 482.808 | 674.397 | 1002.826 | 1090.401 | 2.1 | 1746 |
| mobs-25k-mod | 25000 | 0 | 255.946 | 423.767 | 590.401 | 673.813 | 3.9 | 1279 |
| spread-100k-mod | 100000 | 0 | 143.360 | 228.898 | 348.906 | 413.264 | 7.0 | 2273 |
| spread-100k | 100000 | 0 | 293.252 | 352.698 | 503.353 | 539.923 | 3.4 | 2055 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

