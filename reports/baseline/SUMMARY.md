# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.141 | 76.579 | 138.945 | 177.928 | 16.1 | 1151 |
| mobs-25k | 25000 | 0 | 312.054 | 354.377 | 398.344 | 408.783 | 3.2 | 1345 |
| mixed-150k | 150000 | 0 | 487.349 | 597.915 | 857.736 | 1015.170 | 2.1 | 1477 |
| items-100k-mod | 100000 | 0 | 25.588 | 76.326 | 132.343 | 159.053 | 20.0 | 1360 |
| mixed-150k-mod | 150000 | 0 | 235.880 | 332.004 | 454.847 | 556.726 | 4.2 | 1675 |
| mobs-25k-mod | 25000 | 0 | 196.010 | 291.712 | 415.231 | 516.364 | 5.1 | 1650 |
| spread-100k-mod | 100000 | 0 | 254.803 | 382.266 | 509.483 | 614.288 | 3.9 | 2801 |
| spread-100k | 100000 | 0 | 389.540 | 452.342 | 664.602 | 743.453 | 2.6 | 2056 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

