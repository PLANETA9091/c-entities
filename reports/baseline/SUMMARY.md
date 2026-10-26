# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 59.512 | 73.526 | 130.312 | 176.884 | 16.8 | 1327 |
| mobs-25k | 25000 | 0 | 358.460 | 400.930 | 448.098 | 470.918 | 2.8 | 1806 |
| mixed-150k | 150000 | 0 | 492.349 | 593.374 | 794.874 | 853.041 | 2.0 | 2930 |
| items-100k-mod | 100000 | 0 | 12.801 | 16.949 | 67.585 | 107.895 | 20.0 | 1404 |
| mixed-150k-mod | 150000 | 0 | 255.691 | 310.571 | 361.033 | 389.276 | 3.9 | 1934 |
| mobs-25k-mod | 25000 | 0 | 178.799 | 261.460 | 350.533 | 405.895 | 5.6 | 1602 |
| spread-100k-mod | 100000 | 0 | 140.376 | 212.423 | 279.757 | 398.413 | 7.1 | 2707 |
| spread-100k | 100000 | 0 | 404.411 | 476.400 | 687.974 | 797.461 | 2.5 | 2099 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

