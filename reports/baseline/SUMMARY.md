# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.678 | 91.791 | 166.299 | 200.508 | 14.0 | 2082 |
| mobs-25k | 25000 | 0 | 366.467 | 414.712 | 459.354 | 504.363 | 2.7 | 1827 |
| mixed-150k | 150000 | 0 | 497.334 | 599.583 | 869.854 | 969.016 | 2.0 | 2911 |
| items-100k-mod | 100000 | 0 | 15.495 | 19.396 | 37.160 | 128.498 | 20.0 | 1296 |
| mixed-150k-mod | 150000 | 0 | 265.826 | 330.720 | 383.523 | 461.028 | 3.8 | 2706 |
| mobs-25k-mod | 25000 | 0 | 180.514 | 279.503 | 351.713 | 379.457 | 5.5 | 1851 |
| spread-100k-mod | 100000 | 0 | 143.237 | 215.352 | 301.323 | 392.103 | 7.0 | 2794 |
| spread-100k | 100000 | 0 | 395.637 | 457.809 | 661.832 | 694.840 | 2.5 | 2064 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

