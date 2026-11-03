# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 76.058 | 98.095 | 172.396 | 214.747 | 13.1 | 1491 |
| mobs-25k | 25000 | 0 | 537.422 | 620.064 | 999.073 | 1090.120 | 1.9 | 1462 |
| mixed-150k | 150000 | 0 | 759.744 | 954.858 | 1543.110 | 1766.819 | 1.3 | 2638 |
| items-100k-mod | 100000 | 0 | 29.766 | 76.803 | 175.406 | 223.314 | 20.0 | 1438 |
| mixed-150k-mod | 150000 | 0 | 292.611 | 395.253 | 585.170 | 660.849 | 3.4 | 1982 |
| mobs-25k-mod | 25000 | 0 | 300.322 | 544.406 | 676.805 | 753.981 | 3.3 | 1852 |
| spread-100k-mod | 100000 | 0 | 287.309 | 465.194 | 572.764 | 822.899 | 3.5 | 2886 |
| spread-100k | 100000 | 0 | 513.049 | 585.673 | 820.938 | 862.177 | 1.9 | 2188 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

