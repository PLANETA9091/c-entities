# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 57.359 | 74.014 | 139.111 | 168.444 | 17.4 | 1431 |
| mobs-25k | 25000 | 0 | 381.168 | 430.477 | 472.145 | 490.850 | 2.6 | 1357 |
| mixed-150k | 150000 | 0 | 422.679 | 508.813 | 700.760 | 787.183 | 2.4 | 2234 |
| items-100k-mod | 100000 | 0 | 8.740 | 12.854 | 60.475 | 71.927 | 20.0 | 1349 |
| mixed-150k-mod | 150000 | 0 | 255.598 | 345.656 | 463.656 | 570.743 | 3.9 | 1999 |
| mobs-25k-mod | 25000 | 0 | 197.702 | 347.203 | 548.447 | 663.128 | 5.1 | 1551 |
| spread-100k-mod | 100000 | 0 | 136.685 | 214.757 | 331.975 | 389.271 | 7.3 | 2216 |
| spread-100k | 100000 | 0 | 308.019 | 372.602 | 521.560 | 601.819 | 3.2 | 2972 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

