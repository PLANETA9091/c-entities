# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 61.817 | 78.762 | 145.089 | 209.082 | 16.2 | 1126 |
| mobs-25k | 25000 | 0 | 318.932 | 364.829 | 402.979 | 427.914 | 3.1 | 1214 |
| mixed-150k | 150000 | 0 | 615.731 | 736.895 | 939.372 | 1059.090 | 1.6 | 2901 |
| items-100k-mod | 100000 | 0 | 12.197 | 16.915 | 78.118 | 119.510 | 20.0 | 1428 |
| mixed-150k-mod | 150000 | 0 | 263.229 | 346.429 | 424.033 | 482.503 | 3.8 | 1704 |
| mobs-25k-mod | 25000 | 0 | 182.526 | 299.613 | 485.948 | 553.072 | 5.5 | 1475 |
| spread-100k-mod | 100000 | 0 | 149.415 | 246.694 | 361.545 | 506.433 | 6.7 | 1992 |
| spread-100k | 100000 | 0 | 358.443 | 462.816 | 628.583 | 710.805 | 2.8 | 2100 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

