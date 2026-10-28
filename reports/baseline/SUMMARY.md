# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.874 | 77.484 | 139.846 | 205.633 | 15.9 | 2041 |
| mobs-25k | 25000 | 0 | 382.005 | 438.676 | 483.291 | 550.616 | 2.6 | 935 |
| mixed-150k | 150000 | 0 | 505.859 | 610.379 | 878.415 | 1041.276 | 2.0 | 3105 |
| items-100k-mod | 100000 | 0 | 30.429 | 83.967 | 158.391 | 193.643 | 20.0 | 1640 |
| mixed-150k-mod | 150000 | 0 | 411.992 | 545.480 | 720.480 | 803.210 | 2.4 | 1901 |
| mobs-25k-mod | 25000 | 0 | 246.546 | 418.128 | 644.011 | 853.571 | 4.1 | 1215 |
| spread-100k-mod | 100000 | 0 | 180.081 | 296.736 | 422.303 | 481.484 | 5.6 | 2805 |
| spread-100k | 100000 | 0 | 371.508 | 432.801 | 648.078 | 698.229 | 2.7 | 2115 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

