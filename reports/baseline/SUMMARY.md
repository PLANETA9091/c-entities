# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.434 | 78.674 | 143.333 | 194.184 | 15.5 | 2076 |
| mobs-25k | 25000 | 0 | 309.353 | 350.081 | 390.800 | 411.869 | 3.2 | 1860 |
| mixed-150k | 150000 | 0 | 528.864 | 648.645 | 992.385 | 1148.907 | 1.9 | 3015 |
| items-100k-mod | 100000 | 0 | 13.895 | 16.556 | 82.089 | 114.845 | 20.0 | 1245 |
| mixed-150k-mod | 150000 | 0 | 295.179 | 351.685 | 403.081 | 452.007 | 3.4 | 2371 |
| mobs-25k-mod | 25000 | 0 | 130.902 | 232.989 | 308.937 | 366.665 | 7.6 | 1808 |
| spread-100k-mod | 100000 | 0 | 134.981 | 190.861 | 244.893 | 300.890 | 7.4 | 2037 |
| spread-100k | 100000 | 0 | 320.457 | 385.426 | 544.088 | 617.904 | 3.1 | 2285 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

