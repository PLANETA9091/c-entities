# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.991 | 84.593 | 150.688 | 178.298 | 14.7 | 1638 |
| mobs-25k | 25000 | 0 | 329.660 | 370.529 | 421.831 | 1221.019 | 3.0 | 1433 |
| mixed-150k | 150000 | 0 | 506.905 | 619.295 | 903.177 | 1007.521 | 2.0 | 3231 |
| items-100k-mod | 100000 | 0 | 13.320 | 17.241 | 78.710 | 93.050 | 20.0 | 1183 |
| mixed-150k-mod | 150000 | 0 | 292.791 | 379.482 | 555.069 | 674.698 | 3.4 | 1900 |
| mobs-25k-mod | 25000 | 0 | 151.424 | 271.947 | 489.655 | 571.225 | 6.6 | 1250 |
| spread-100k-mod | 100000 | 0 | 175.607 | 334.879 | 485.230 | 686.167 | 5.7 | 1854 |
| spread-100k | 100000 | 0 | 536.360 | 613.226 | 858.451 | 929.747 | 1.9 | 2122 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

