# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 53.324 | 91.016 | 138.900 | 204.339 | 18.8 | 1380 |
| mobs-25k | 25000 | 0 | 327.277 | 393.337 | 481.357 | 601.862 | 3.1 | 1851 |
| mixed-150k | 150000 | 0 | 415.162 | 571.101 | 776.437 | 980.324 | 2.4 | 2900 |
| items-100k-mod | 100000 | 0 | 25.073 | 40.206 | 106.283 | 236.707 | 20.0 | 1687 |
| mixed-150k-mod | 150000 | 0 | 85.417 | 138.178 | 274.232 | 417.381 | 11.7 | 2332 |
| mobs-25k-mod | 25000 | 0 | 1987.037 | 2145.693 | 2243.484 | 2332.811 | 0.5 | 2107 |
| mobs-25k-native-mod | 25000 | 0 | 21.325 | 33.039 | 114.617 | 136.060 | 20.0 | 1772 |
| mobs-25k-native | 25000 | 0 | 323.066 | 396.212 | 484.565 | 560.783 | 3.1 | 967 |
| spread-100k-mod | 100000 | 0 | 63.942 | 122.475 | 232.725 | 323.429 | 15.6 | 2343 |
| spread-100k | 100000 | 0 | 555.511 | 641.118 | 748.865 | 794.762 | 1.8 | 2245 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

