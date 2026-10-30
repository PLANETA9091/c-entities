# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 61.698 | 81.404 | 153.269 | 230.923 | 16.2 | 1055 |
| mobs-25k | 25000 | 0 | 414.746 | 471.143 | 517.955 | 563.517 | 2.4 | 1600 |
| mixed-150k | 150000 | 0 | 642.438 | 809.139 | 1112.142 | 1355.470 | 1.6 | 3269 |
| items-100k-mod | 100000 | 0 | 32.401 | 74.383 | 155.972 | 335.649 | 20.0 | 1055 |
| mixed-150k-mod | 150000 | 0 | 259.727 | 382.464 | 514.966 | 672.236 | 3.9 | 1812 |
| mobs-25k-mod | 25000 | 0 | 241.661 | 468.696 | 604.447 | 668.033 | 4.1 | 1440 |
| spread-100k-mod | 100000 | 0 | 276.117 | 451.822 | 591.453 | 826.311 | 3.6 | 1994 |
| spread-100k | 100000 | 0 | 481.355 | 552.728 | 754.138 | 809.641 | 2.1 | 2154 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

