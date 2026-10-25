# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.501 | 79.902 | 144.166 | 207.471 | 15.5 | 1068 |
| mobs-25k | 25000 | 0 | 305.950 | 345.924 | 383.804 | 437.537 | 3.3 | 1260 |
| mixed-150k | 150000 | 0 | 481.613 | 590.629 | 791.648 | 886.475 | 2.1 | 2328 |
| items-100k-mod | 100000 | 0 | 41.728 | 61.149 | 132.589 | 213.200 | 20.0 | 1024 |
| mixed-150k-mod | 150000 | 0 | 350.654 | 426.657 | 474.858 | 499.907 | 2.9 | 2017 |
| mobs-25k-mod | 25000 | 0 | 144.911 | 272.078 | 356.041 | 378.938 | 6.9 | 1614 |
| spread-100k-mod | 100000 | 0 | 146.911 | 231.056 | 309.063 | 406.013 | 6.8 | 1735 |
| spread-100k | 100000 | 0 | 334.803 | 389.611 | 570.019 | 610.975 | 3.0 | 1776 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

