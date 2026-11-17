# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.434 | 112.223 | 168.850 | 235.372 | 14.4 | 1213 |
| mobs-25k | 25000 | 0 | 516.796 | 595.302 | 679.965 | 789.125 | 1.9 | 1236 |
| mixed-150k | 150000 | 0 | 407.538 | 563.954 | 790.732 | 1075.885 | 2.5 | 2832 |
| items-100k-mod | 100000 | 0 | 17.449 | 30.695 | 103.060 | 212.093 | 20.0 | 1849 |
| mixed-150k-mod | 150000 | 0 | 62.028 | 100.453 | 230.022 | 323.308 | 16.1 | 2403 |
| mobs-25k-mod | 25000 | 0 | 369.816 | 2085.304 | 3079.285 | 3392.747 | 2.7 | 2172 |
| mobs-25k-native-mod | 25000 | 0 | 23.534 | 46.214 | 141.107 | 243.299 | 20.0 | 1676 |
| mobs-25k-native | 25000 | 0 | 563.261 | 634.667 | 692.768 | 741.962 | 1.8 | 1232 |
| spread-100k-mod | 100000 | 0 | 54.809 | 78.718 | 209.673 | 292.024 | 18.2 | 2184 |
| spread-100k | 100000 | 0 | 399.406 | 485.821 | 583.734 | 745.380 | 2.5 | 2398 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

