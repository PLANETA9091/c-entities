# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.212 | 84.006 | 156.095 | 212.078 | 14.9 | 798 |
| mobs-25k | 25000 | 0 | 742.623 | 822.447 | 899.632 | 947.102 | 1.3 | 1604 |
| mixed-150k | 150000 | 0 | 457.448 | 557.566 | 804.306 | 1027.088 | 2.2 | 2731 |
| items-100k-mod | 100000 | 0 | 26.585 | 82.730 | 153.770 | 178.942 | 20.0 | 1579 |
| mixed-150k-mod | 150000 | 0 | 319.103 | 411.853 | 524.393 | 620.160 | 3.1 | 1995 |
| mobs-25k-mod | 25000 | 0 | 334.411 | 506.313 | 641.422 | 683.637 | 3.0 | 1827 |
| spread-100k-mod | 100000 | 0 | 395.523 | 523.775 | 679.604 | 926.717 | 2.5 | 2059 |
| spread-100k | 100000 | 0 | 512.380 | 591.765 | 824.432 | 891.119 | 2.0 | 2049 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

