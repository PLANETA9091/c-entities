# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.710 | 84.566 | 154.829 | 212.308 | 14.6 | 1862 |
| mobs-25k | 25000 | 0 | 521.096 | 570.352 | 629.261 | 664.624 | 1.9 | 851 |
| mixed-150k | 150000 | 0 | 489.073 | 614.389 | 902.671 | 1086.260 | 2.0 | 3226 |
| items-100k-mod | 100000 | 0 | 30.492 | 87.063 | 159.146 | 282.026 | 20.0 | 1153 |
| mixed-150k-mod | 150000 | 0 | 254.424 | 375.297 | 608.014 | 723.697 | 3.9 | 2011 |
| mobs-25k-mod | 25000 | 0 | 267.616 | 437.723 | 543.416 | 649.569 | 3.7 | 1333 |
| spread-100k-mod | 100000 | 0 | 298.529 | 461.658 | 657.666 | 766.513 | 3.3 | 1989 |
| spread-100k | 100000 | 0 | 514.653 | 577.969 | 807.727 | 861.840 | 1.9 | 2091 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

