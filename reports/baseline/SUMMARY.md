# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.514 | 85.941 | 155.507 | 205.978 | 14.4 | 1213 |
| mobs-25k | 25000 | 0 | 517.753 | 577.204 | 627.654 | 803.636 | 1.9 | 1780 |
| mixed-150k | 150000 | 0 | 649.310 | 758.290 | 1004.085 | 1102.775 | 1.5 | 2568 |
| items-100k-mod | 100000 | 0 | 34.660 | 89.797 | 156.386 | 190.643 | 20.0 | 1434 |
| mixed-150k-mod | 150000 | 0 | 278.707 | 399.830 | 511.303 | 627.069 | 3.6 | 1681 |
| mobs-25k-mod | 25000 | 0 | 263.433 | 401.340 | 531.752 | 691.151 | 3.8 | 1499 |
| spread-100k-mod | 100000 | 0 | 269.339 | 416.820 | 535.002 | 635.315 | 3.7 | 2748 |
| spread-100k | 100000 | 0 | 431.475 | 500.093 | 697.646 | 787.553 | 2.3 | 2223 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

