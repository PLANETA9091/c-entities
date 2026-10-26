# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.613 | 84.914 | 156.641 | 197.553 | 14.8 | 1218 |
| mobs-25k | 25000 | 0 | 306.566 | 344.386 | 381.715 | 402.721 | 3.3 | 1676 |
| mixed-150k | 150000 | 0 | 518.276 | 757.290 | 1032.613 | 1367.522 | 1.9 | 3314 |
| items-100k-mod | 100000 | 0 | 13.487 | 18.485 | 79.096 | 113.252 | 20.0 | 1344 |
| mixed-150k-mod | 150000 | 0 | 322.814 | 415.817 | 471.412 | 505.929 | 3.1 | 2184 |
| mobs-25k-mod | 25000 | 0 | 156.371 | 271.916 | 328.013 | 399.652 | 6.4 | 1376 |
| spread-100k-mod | 100000 | 0 | 120.776 | 190.480 | 276.057 | 360.982 | 8.3 | 1878 |
| spread-100k | 100000 | 0 | 322.252 | 382.306 | 542.136 | 572.107 | 3.1 | 2980 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

