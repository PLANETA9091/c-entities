# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.492 | 81.587 | 151.104 | 187.020 | 15.0 | 1756 |
| mobs-25k | 25000 | 0 | 332.480 | 377.598 | 410.141 | 441.771 | 3.0 | 1094 |
| mixed-150k | 150000 | 0 | 536.277 | 668.941 | 961.302 | 1042.313 | 1.9 | 2152 |
| items-100k-mod | 100000 | 0 | 18.845 | 23.405 | 98.247 | 111.832 | 20.0 | 1151 |
| mixed-150k-mod | 150000 | 0 | 294.249 | 349.120 | 406.033 | 462.597 | 3.4 | 1541 |
| mobs-25k-mod | 25000 | 0 | 148.874 | 266.690 | 343.457 | 410.517 | 6.7 | 1710 |
| spread-100k-mod | 100000 | 0 | 123.129 | 200.134 | 268.275 | 374.462 | 8.1 | 2634 |
| spread-100k | 100000 | 0 | 366.193 | 410.553 | 587.579 | 634.154 | 2.7 | 1766 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

