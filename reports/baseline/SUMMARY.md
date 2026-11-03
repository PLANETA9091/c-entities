# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.661 | 82.432 | 155.650 | 200.279 | 15.0 | 1183 |
| mobs-25k | 25000 | 0 | 412.655 | 463.209 | 516.089 | 557.734 | 2.4 | 1133 |
| mixed-150k | 150000 | 0 | 898.437 | 1064.505 | 1459.508 | 1545.645 | 1.1 | 2549 |
| items-100k-mod | 100000 | 0 | 27.999 | 70.246 | 160.378 | 215.649 | 20.0 | 1604 |
| mixed-150k-mod | 150000 | 0 | 237.156 | 348.366 | 490.815 | 623.077 | 4.2 | 1857 |
| mobs-25k-mod | 25000 | 0 | 233.924 | 396.182 | 543.070 | 608.979 | 4.3 | 1642 |
| spread-100k-mod | 100000 | 0 | 330.844 | 509.813 | 669.626 | 1014.939 | 3.0 | 2058 |
| spread-100k | 100000 | 0 | 566.466 | 644.058 | 910.787 | 991.263 | 1.8 | 2197 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

