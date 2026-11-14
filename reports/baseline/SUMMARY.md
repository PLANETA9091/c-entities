# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.759 | 110.432 | 178.413 | 244.752 | 14.3 | 874 |
| mobs-25k | 25000 | 0 | 318.906 | 381.382 | 473.632 | 595.256 | 3.1 | 1121 |
| mixed-150k | 150000 | 0 | 470.887 | 641.525 | 834.016 | 907.705 | 2.1 | 3008 |
| items-100k-mod | 100000 | 0 | 27.798 | 43.724 | 124.595 | 307.576 | 20.0 | 1591 |
| mixed-150k-mod | 150000 | 0 | 175.044 | 318.729 | 506.281 | 605.159 | 5.7 | 2491 |
| mobs-25k-mod | 25000 | 0 | 1752.920 | 1862.213 | 1970.577 | 2048.115 | 0.6 | 2248 |
| mobs-25k-native-mod | 25000 | 0 | 16.998 | 33.975 | 103.256 | 137.005 | 20.0 | 1536 |
| mobs-25k-native | 25000 | 0 | 461.843 | 526.980 | 619.409 | 687.334 | 2.2 | 1554 |
| spread-100k-mod | 100000 | 0 | 167.549 | 253.986 | 378.365 | 460.811 | 6.0 | 2753 |
| spread-100k | 100000 | 0 | 363.556 | 459.673 | 565.307 | 651.835 | 2.8 | 1918 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

