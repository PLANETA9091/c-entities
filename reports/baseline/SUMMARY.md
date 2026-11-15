# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 75.266 | 136.036 | 202.899 | 278.650 | 13.3 | 2303 |
| mobs-25k | 25000 | 0 | 531.014 | 604.353 | 695.340 | 939.140 | 1.9 | 1502 |
| mixed-150k | 150000 | 0 | 589.190 | 883.544 | 1228.461 | 1608.651 | 1.7 | 2492 |
| items-100k-mod | 100000 | 0 | 36.214 | 62.370 | 163.714 | 250.756 | 20.0 | 1552 |
| mixed-150k-mod | 150000 | 0 | 97.051 | 174.997 | 330.570 | 434.658 | 10.3 | 2038 |
| mobs-25k-mod | 25000 | 0 | 2576.183 | 2779.935 | 2991.021 | 3128.078 | 0.4 | 2308 |
| mobs-25k-native-mod | 25000 | 0 | 25.030 | 41.516 | 117.552 | 183.768 | 20.0 | 1665 |
| mobs-25k-native | 25000 | 0 | 397.961 | 468.896 | 564.724 | 641.252 | 2.5 | 1925 |
| spread-100k-mod | 100000 | 0 | 41.265 | 63.596 | 167.327 | 234.563 | 20.0 | 2464 |
| spread-100k | 100000 | 0 | 350.510 | 446.531 | 551.124 | 691.548 | 2.9 | 1984 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

