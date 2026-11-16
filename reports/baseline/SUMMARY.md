# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 79.632 | 147.567 | 209.370 | 264.595 | 12.6 | 1758 |
| mobs-25k | 25000 | 0 | 495.930 | 571.537 | 664.535 | 760.080 | 2.0 | 1778 |
| mixed-150k | 150000 | 0 | 785.787 | 1138.472 | 1673.159 | 1994.123 | 1.3 | 2414 |
| items-100k-mod | 100000 | 0 | 34.952 | 66.589 | 136.561 | 316.987 | 20.0 | 1943 |
| mixed-150k-mod | 150000 | 0 | 109.483 | 195.989 | 382.322 | 550.837 | 9.1 | 2104 |
| mobs-25k-mod | 25000 | 0 | 333.537 | 1791.392 | 2763.223 | 3158.985 | 3.0 | 1945 |
| mobs-25k-native-mod | 25000 | 0 | 25.127 | 44.096 | 115.229 | 212.713 | 20.0 | 1685 |
| mobs-25k-native | 25000 | 0 | 712.947 | 800.773 | 878.985 | 1026.220 | 1.4 | 1220 |
| spread-100k-mod | 100000 | 0 | 63.741 | 122.897 | 223.457 | 286.794 | 15.7 | 2305 |
| spread-100k | 100000 | 0 | 452.150 | 559.899 | 680.651 | 772.694 | 2.2 | 1944 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

