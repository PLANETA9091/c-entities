# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 56.801 | 73.072 | 131.267 | 181.670 | 17.6 | 1623 |
| mobs-25k | 25000 | 0 | 356.981 | 405.618 | 448.603 | 496.481 | 2.8 | 1248 |
| mixed-150k | 150000 | 0 | 433.343 | 527.538 | 730.860 | 828.440 | 2.3 | 2602 |
| items-100k-mod | 100000 | 0 | 29.231 | 50.732 | 122.886 | 176.326 | 20.0 | 997 |
| mixed-150k-mod | 150000 | 0 | 316.052 | 383.325 | 426.565 | 467.848 | 3.2 | 2128 |
| mobs-25k-mod | 25000 | 0 | 191.611 | 319.114 | 395.926 | 453.949 | 5.2 | 1412 |
| spread-100k-mod | 100000 | 0 | 151.880 | 255.522 | 360.769 | 462.290 | 6.6 | 1850 |
| spread-100k | 100000 | 0 | 447.756 | 517.050 | 727.735 | 768.985 | 2.2 | 2328 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

