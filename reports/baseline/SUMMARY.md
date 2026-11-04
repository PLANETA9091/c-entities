# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.542 | 81.991 | 151.765 | 210.603 | 15.0 | 1541 |
| mobs-25k | 25000 | 0 | 391.056 | 435.382 | 468.311 | 496.846 | 2.6 | 885 |
| mixed-150k | 150000 | 0 | 613.730 | 723.467 | 947.551 | 1037.532 | 1.6 | 2748 |
| items-100k-mod | 100000 | 0 | 25.313 | 55.748 | 129.842 | 210.473 | 20.0 | 1509 |
| mixed-150k-mod | 150000 | 0 | 290.461 | 375.910 | 494.727 | 596.358 | 3.4 | 2208 |
| mobs-25k-mod | 25000 | 0 | 257.742 | 453.595 | 628.854 | 755.020 | 3.9 | 1958 |
| spread-100k-mod | 100000 | 0 | 351.523 | 459.191 | 594.886 | 746.934 | 2.8 | 2819 |
| spread-100k | 100000 | 0 | 491.598 | 561.190 | 759.854 | 825.805 | 2.0 | 2073 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

