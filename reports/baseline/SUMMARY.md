# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.425 | 83.967 | 148.735 | 206.652 | 14.8 | 1599 |
| mobs-25k | 25000 | 0 | 499.393 | 552.411 | 599.422 | 663.155 | 2.0 | 1021 |
| mixed-150k | 150000 | 0 | 679.946 | 854.675 | 1141.373 | 1366.544 | 1.5 | 2065 |
| items-100k-mod | 100000 | 0 | 24.612 | 56.789 | 135.367 | 241.840 | 20.0 | 1381 |
| mixed-150k-mod | 150000 | 0 | 267.046 | 384.079 | 570.206 | 656.580 | 3.7 | 1999 |
| mobs-25k-mod | 25000 | 0 | 290.399 | 533.663 | 656.611 | 763.225 | 3.4 | 1537 |
| spread-100k-mod | 100000 | 0 | 337.878 | 523.910 | 674.319 | 927.654 | 3.0 | 2024 |
| spread-100k | 100000 | 0 | 577.818 | 674.754 | 923.994 | 1001.979 | 1.7 | 2635 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

