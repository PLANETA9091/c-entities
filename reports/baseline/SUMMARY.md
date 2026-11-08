# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.157 | 107.546 | 163.552 | 218.219 | 14.5 | 909 |
| mobs-25k | 25000 | 0 | 522.178 | 588.525 | 696.028 | 839.120 | 1.9 | 1528 |
| mixed-150k | 150000 | 0 | 557.899 | 773.156 | 1049.983 | 1167.013 | 1.8 | 1902 |
| items-100k-mod | 100000 | 0 | 25.322 | 40.840 | 101.760 | 122.683 | 20.0 | 1472 |
| mixed-150k-mod | 150000 | 0 | 1374.200 | 1548.426 | 1716.062 | 1776.777 | 0.7 | 1820 |
| mobs-25k-mod | 25000 | 0 | 954.072 | 1060.555 | 1427.453 | 1603.266 | 1.0 | 2143 |
| spread-100k-mod | 100000 | 0 | 725.199 | 845.480 | 1097.592 | 1360.922 | 1.4 | 2453 |
| spread-100k | 100000 | 0 | 474.407 | 584.823 | 794.022 | 945.134 | 2.1 | 2233 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

