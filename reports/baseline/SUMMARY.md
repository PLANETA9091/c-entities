# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 73.772 | 90.267 | 167.623 | 227.603 | 13.6 | 1443 |
| mobs-25k | 25000 | 0 | 513.456 | 568.018 | 612.617 | 643.326 | 1.9 | 861 |
| mixed-150k | 150000 | 0 | 898.613 | 1099.746 | 1532.692 | 1681.856 | 1.1 | 2964 |
| items-100k-mod | 100000 | 0 | 37.158 | 99.539 | 181.663 | 237.760 | 20.0 | 1208 |
| mixed-150k-mod | 150000 | 0 | 372.014 | 469.156 | 626.710 | 718.245 | 2.7 | 1731 |
| mobs-25k-mod | 25000 | 0 | 193.587 | 432.833 | 693.020 | 856.457 | 5.2 | 1298 |
| spread-100k-mod | 100000 | 0 | 163.853 | 279.426 | 432.840 | 536.961 | 6.1 | 1905 |
| spread-100k | 100000 | 0 | 366.699 | 430.730 | 617.449 | 646.386 | 2.7 | 1957 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

