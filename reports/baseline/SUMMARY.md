# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 75.669 | 88.776 | 174.306 | 265.837 | 13.2 | 1072 |
| mobs-25k | 25000 | 0 | 411.159 | 461.619 | 507.064 | 533.596 | 2.4 | 931 |
| mixed-150k | 150000 | 0 | 887.089 | 1082.034 | 1441.123 | 1672.119 | 1.1 | 2801 |
| items-100k-mod | 100000 | 0 | 30.516 | 85.781 | 174.568 | 229.905 | 20.0 | 1346 |
| mixed-150k-mod | 150000 | 0 | 233.837 | 339.403 | 498.915 | 574.653 | 4.3 | 2019 |
| mobs-25k-mod | 25000 | 0 | 227.805 | 399.026 | 524.669 | 592.951 | 4.4 | 1420 |
| spread-100k-mod | 100000 | 0 | 248.652 | 373.437 | 527.256 | 641.076 | 4.0 | 2885 |
| spread-100k | 100000 | 0 | 380.563 | 442.613 | 625.249 | 675.787 | 2.6 | 2099 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

