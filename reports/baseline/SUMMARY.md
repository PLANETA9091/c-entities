# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.612 | 85.894 | 158.213 | 213.427 | 14.8 | 841 |
| mobs-25k | 25000 | 0 | 571.696 | 627.559 | 696.043 | 713.060 | 1.7 | 899 |
| mixed-150k | 150000 | 0 | 531.059 | 635.560 | 852.460 | 985.870 | 1.9 | 2718 |
| items-100k-mod | 100000 | 0 | 34.457 | 91.479 | 149.043 | 285.113 | 20.0 | 1088 |
| mixed-150k-mod | 150000 | 0 | 235.838 | 342.095 | 469.562 | 564.732 | 4.2 | 1871 |
| mobs-25k-mod | 25000 | 0 | 247.998 | 445.726 | 553.290 | 690.528 | 4.0 | 1854 |
| spread-100k-mod | 100000 | 0 | 249.476 | 382.797 | 493.356 | 628.862 | 4.0 | 1864 |
| spread-100k | 100000 | 0 | 374.718 | 441.548 | 629.091 | 680.152 | 2.7 | 2105 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

