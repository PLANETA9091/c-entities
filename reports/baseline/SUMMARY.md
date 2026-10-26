# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.916 | 83.675 | 158.462 | 216.529 | 14.7 | 855 |
| mobs-25k | 25000 | 0 | 242.168 | 273.891 | 313.755 | 365.814 | 4.1 | 918 |
| mixed-150k | 150000 | 0 | 389.649 | 477.455 | 685.311 | 798.806 | 2.6 | 2435 |
| items-100k-mod | 100000 | 0 | 15.325 | 19.644 | 85.879 | 155.024 | 20.0 | 1075 |
| mixed-150k-mod | 150000 | 0 | 264.646 | 313.202 | 363.636 | 419.413 | 3.8 | 1782 |
| mobs-25k-mod | 25000 | 0 | 149.038 | 214.916 | 274.887 | 285.944 | 6.7 | 1140 |
| spread-100k-mod | 100000 | 0 | 150.562 | 222.857 | 286.951 | 355.697 | 6.6 | 2016 |
| spread-100k | 100000 | 0 | 397.723 | 455.954 | 646.224 | 761.744 | 2.5 | 1980 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

