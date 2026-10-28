# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.499 | 87.224 | 156.296 | 204.916 | 14.2 | 931 |
| mobs-25k | 25000 | 0 | 335.448 | 377.147 | 416.697 | 449.965 | 3.0 | 866 |
| mixed-150k | 150000 | 0 | 506.340 | 618.432 | 825.713 | 924.745 | 2.0 | 2856 |
| items-100k-mod | 100000 | 0 | 35.285 | 97.241 | 163.564 | 238.110 | 20.0 | 1399 |
| mixed-150k-mod | 150000 | 0 | 370.014 | 512.652 | 660.261 | 716.590 | 2.7 | 1770 |
| mobs-25k-mod | 25000 | 0 | 172.355 | 296.601 | 484.236 | 569.328 | 5.8 | 1194 |
| spread-100k-mod | 100000 | 0 | 228.947 | 396.016 | 569.806 | 791.106 | 4.4 | 1859 |
| spread-100k | 100000 | 0 | 545.565 | 607.740 | 877.611 | 954.785 | 1.8 | 2071 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

