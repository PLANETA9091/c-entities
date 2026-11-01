# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 61.044 | 77.593 | 140.068 | 178.835 | 16.4 | 1084 |
| mobs-25k | 25000 | 0 | 560.021 | 624.642 | 673.119 | 731.527 | 1.8 | 1255 |
| mixed-150k | 150000 | 0 | 627.112 | 741.943 | 956.197 | 1046.831 | 1.6 | 3133 |
| items-100k-mod | 100000 | 0 | 25.094 | 61.828 | 134.234 | 180.575 | 20.0 | 1521 |
| mixed-150k-mod | 150000 | 0 | 229.806 | 325.945 | 412.592 | 472.048 | 4.4 | 1697 |
| mobs-25k-mod | 25000 | 0 | 308.893 | 577.272 | 734.276 | 845.389 | 3.2 | 1879 |
| spread-100k-mod | 100000 | 0 | 308.289 | 457.211 | 608.125 | 734.895 | 3.2 | 2209 |
| spread-100k | 100000 | 0 | 524.052 | 596.527 | 821.198 | 987.435 | 1.9 | 2120 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

