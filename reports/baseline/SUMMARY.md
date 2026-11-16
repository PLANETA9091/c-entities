# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.041 | 114.887 | 196.151 | 370.357 | 14.1 | 988 |
| mobs-25k | 25000 | 0 | 343.177 | 415.760 | 517.173 | 570.408 | 2.9 | 1540 |
| mixed-150k | 150000 | 0 | 568.136 | 766.360 | 985.818 | 1105.543 | 1.8 | 2073 |
| items-100k-mod | 100000 | 0 | 30.596 | 53.458 | 127.553 | 372.887 | 20.0 | 1626 |
| mixed-150k-mod | 150000 | 0 | 90.057 | 162.281 | 319.647 | 425.234 | 11.1 | 2118 |
| mobs-25k-mod | 25000 | 0 | 270.286 | 1255.307 | 2162.248 | 2248.419 | 3.7 | 1942 |
| mobs-25k-native-mod | 25000 | 0 | 22.431 | 41.003 | 124.287 | 252.729 | 20.0 | 1938 |
| mobs-25k-native | 25000 | 0 | 413.183 | 486.673 | 593.064 | 754.400 | 2.4 | 1690 |
| spread-100k-mod | 100000 | 0 | 59.988 | 106.538 | 198.453 | 296.398 | 16.7 | 2401 |
| spread-100k | 100000 | 0 | 520.849 | 618.353 | 692.352 | 812.095 | 1.9 | 1993 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

