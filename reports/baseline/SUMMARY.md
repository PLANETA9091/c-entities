# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.392 | 80.305 | 149.691 | 192.791 | 15.5 | 927 |
| mobs-25k | 25000 | 0 | 516.389 | 571.066 | 623.934 | 671.060 | 1.9 | 1522 |
| mixed-150k | 150000 | 0 | 454.523 | 563.089 | 784.216 | 878.344 | 2.2 | 2888 |
| items-100k-mod | 100000 | 0 | 36.573 | 85.798 | 149.227 | 206.117 | 20.0 | 1183 |
| mixed-150k-mod | 150000 | 0 | 231.662 | 328.334 | 457.307 | 608.244 | 4.3 | 2546 |
| mobs-25k-mod | 25000 | 0 | 293.450 | 550.694 | 757.012 | 819.615 | 3.4 | 1266 |
| spread-100k-mod | 100000 | 0 | 380.654 | 518.832 | 734.569 | 1108.318 | 2.6 | 1841 |
| spread-100k | 100000 | 0 | 627.875 | 730.181 | 1088.633 | 1298.382 | 1.6 | 2117 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

