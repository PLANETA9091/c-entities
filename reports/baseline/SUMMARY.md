# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.928 | 84.034 | 159.314 | 201.091 | 14.5 | 873 |
| mobs-25k | 25000 | 0 | 534.457 | 587.849 | 631.477 | 648.163 | 1.9 | 1197 |
| mixed-150k | 150000 | 0 | 675.096 | 846.403 | 1245.785 | 1421.124 | 1.5 | 1943 |
| items-100k-mod | 100000 | 0 | 29.592 | 88.641 | 142.480 | 210.332 | 20.0 | 1779 |
| mixed-150k-mod | 150000 | 0 | 312.127 | 418.375 | 656.201 | 738.539 | 3.2 | 1763 |
| mobs-25k-mod | 25000 | 0 | 282.038 | 476.421 | 583.503 | 689.249 | 3.5 | 1293 |
| spread-100k-mod | 100000 | 0 | 241.594 | 385.871 | 524.862 | 671.556 | 4.1 | 2690 |
| spread-100k | 100000 | 0 | 401.900 | 464.148 | 679.901 | 812.300 | 2.5 | 2155 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

