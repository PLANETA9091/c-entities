# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.956 | 79.823 | 140.416 | 201.514 | 15.4 | 1209 |
| mobs-25k | 25000 | 0 | 308.227 | 352.289 | 386.579 | 438.042 | 3.2 | 1042 |
| mixed-150k | 150000 | 0 | 750.279 | 857.622 | 1155.999 | 1232.676 | 1.3 | 2645 |
| items-100k-mod | 100000 | 0 | 28.259 | 76.386 | 137.903 | 186.039 | 20.0 | 1336 |
| mixed-150k-mod | 150000 | 0 | 368.300 | 471.538 | 678.112 | 778.666 | 2.7 | 1868 |
| mobs-25k-mod | 25000 | 0 | 159.818 | 265.054 | 476.388 | 533.872 | 6.3 | 1490 |
| spread-100k-mod | 100000 | 0 | 149.675 | 235.954 | 364.278 | 548.381 | 6.7 | 2458 |
| spread-100k | 100000 | 0 | 337.862 | 401.625 | 577.500 | 652.312 | 3.0 | 1901 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

