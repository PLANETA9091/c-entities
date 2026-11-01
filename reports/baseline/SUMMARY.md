# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.871 | 76.622 | 141.003 | 199.186 | 15.9 | 1435 |
| mobs-25k | 25000 | 0 | 789.568 | 868.948 | 947.435 | 988.520 | 1.3 | 1875 |
| mixed-150k | 150000 | 0 | 664.432 | 794.225 | 1090.726 | 1188.828 | 1.5 | 2124 |
| items-100k-mod | 100000 | 0 | 27.168 | 72.641 | 144.104 | 170.689 | 20.0 | 1529 |
| mixed-150k-mod | 150000 | 0 | 278.209 | 385.538 | 581.449 | 701.469 | 3.6 | 1655 |
| mobs-25k-mod | 25000 | 0 | 276.603 | 492.904 | 682.377 | 786.007 | 3.6 | 1211 |
| spread-100k-mod | 100000 | 0 | 255.532 | 391.125 | 508.973 | 657.018 | 3.9 | 1978 |
| spread-100k | 100000 | 0 | 469.875 | 538.221 | 797.766 | 860.773 | 2.1 | 2104 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

