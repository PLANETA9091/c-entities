# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 56.297 | 98.656 | 150.347 | 211.537 | 17.8 | 798 |
| mobs-25k | 25000 | 0 | 578.280 | 646.209 | 734.647 | 839.324 | 1.7 | 1371 |
| mixed-150k | 150000 | 0 | 551.032 | 767.106 | 1029.321 | 1163.880 | 1.8 | 2122 |
| items-100k-mod | 100000 | 0 | 23.095 | 39.332 | 98.695 | 265.298 | 20.0 | 2053 |
| mixed-150k-mod | 150000 | 0 | 90.921 | 172.141 | 328.418 | 421.816 | 11.0 | 2100 |
| mobs-25k-mod | 25000 | 0 | 2505.247 | 2661.904 | 2813.497 | 2911.427 | 0.4 | 2379 |
| mobs-25k-native-mod | 25000 | 0 | 23.790 | 46.394 | 118.094 | 161.434 | 20.0 | 1750 |
| mobs-25k-native | 25000 | 0 | 493.304 | 566.946 | 670.288 | 818.915 | 2.0 | 1370 |
| spread-100k-mod | 100000 | 0 | 62.393 | 119.000 | 226.379 | 319.637 | 16.0 | 2165 |
| spread-100k | 100000 | 0 | 445.832 | 542.023 | 610.293 | 714.889 | 2.2 | 1864 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

