# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.331 | 85.627 | 150.974 | 201.221 | 15.3 | 1742 |
| mobs-25k | 25000 | 0 | 538.680 | 597.532 | 658.337 | 744.000 | 1.9 | 1994 |
| mixed-150k | 150000 | 0 | 454.882 | 560.960 | 794.489 | 920.785 | 2.2 | 2593 |
| items-100k-mod | 100000 | 0 | 30.323 | 82.551 | 154.210 | 199.974 | 20.0 | 1185 |
| mixed-150k-mod | 150000 | 0 | 379.477 | 546.092 | 712.494 | 756.417 | 2.6 | 2161 |
| mobs-25k-mod | 25000 | 0 | 241.877 | 454.356 | 649.324 | 738.552 | 4.1 | 2113 |
| spread-100k-mod | 100000 | 0 | 174.541 | 292.026 | 439.411 | 568.935 | 5.7 | 1846 |
| spread-100k | 100000 | 0 | 492.322 | 563.888 | 773.813 | 835.972 | 2.0 | 2037 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

