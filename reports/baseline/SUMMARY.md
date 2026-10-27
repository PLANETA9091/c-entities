# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 56.701 | 73.327 | 128.275 | 202.841 | 17.6 | 1270 |
| mobs-25k | 25000 | 0 | 415.542 | 465.087 | 507.403 | 555.078 | 2.4 | 1336 |
| mixed-150k | 150000 | 0 | 531.294 | 649.151 | 920.032 | 1031.571 | 1.9 | 2855 |
| items-100k-mod | 100000 | 0 | 11.109 | 14.445 | 63.749 | 78.285 | 20.0 | 1469 |
| mixed-150k-mod | 150000 | 0 | 330.039 | 430.323 | 629.169 | 789.202 | 3.0 | 1739 |
| mobs-25k-mod | 25000 | 0 | 220.842 | 386.484 | 566.229 | 607.606 | 4.5 | 1611 |
| spread-100k-mod | 100000 | 0 | 169.267 | 287.330 | 419.029 | 489.004 | 5.9 | 2636 |
| spread-100k | 100000 | 0 | 422.356 | 480.478 | 708.773 | 758.144 | 2.4 | 2012 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

