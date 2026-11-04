# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 73.262 | 89.264 | 159.289 | 182.815 | 13.6 | 781 |
| mobs-25k | 25000 | 0 | 550.594 | 617.522 | 664.828 | 719.330 | 1.8 | 996 |
| mixed-150k | 150000 | 0 | 635.639 | 764.691 | 972.819 | 1124.011 | 1.6 | 1675 |
| items-100k-mod | 100000 | 0 | 30.994 | 88.930 | 158.914 | 179.278 | 20.0 | 1258 |
| mixed-150k-mod | 150000 | 0 | 300.264 | 384.067 | 501.572 | 657.433 | 3.3 | 2109 |
| mobs-25k-mod | 25000 | 0 | 358.935 | 561.549 | 678.862 | 781.963 | 2.8 | 1720 |
| spread-100k-mod | 100000 | 0 | 450.034 | 547.762 | 738.141 | 870.277 | 2.2 | 2694 |
| spread-100k | 100000 | 0 | 614.903 | 690.990 | 966.432 | 1029.377 | 1.6 | 2078 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

