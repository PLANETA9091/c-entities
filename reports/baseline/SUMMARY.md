# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 60.410 | 76.624 | 141.777 | 196.386 | 16.6 | 893 |
| mobs-25k | 25000 | 0 | 358.919 | 405.946 | 446.522 | 475.740 | 2.8 | 996 |
| mixed-150k | 150000 | 0 | 453.029 | 539.828 | 757.754 | 850.933 | 2.2 | 2912 |
| items-100k-mod | 100000 | 0 | 55.315 | 71.431 | 186.205 | 226.625 | 18.1 | 1544 |
| mixed-150k-mod | 150000 | 0 | 300.484 | 357.751 | 412.446 | 442.374 | 3.3 | 1905 |
| mobs-25k-mod | 25000 | 0 | 207.434 | 341.345 | 425.224 | 506.898 | 4.8 | 1541 |
| spread-100k-mod | 100000 | 0 | 178.159 | 267.415 | 350.114 | 456.012 | 5.6 | 2442 |
| spread-100k | 100000 | 0 | 382.055 | 449.583 | 632.968 | 677.213 | 2.6 | 2011 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

