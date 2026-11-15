# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.837 | 119.937 | 184.228 | 226.480 | 13.7 | 1570 |
| mobs-25k | 25000 | 0 | 478.890 | 559.907 | 670.144 | 900.998 | 2.1 | 1338 |
| mixed-150k | 150000 | 0 | 469.006 | 652.290 | 835.718 | 981.746 | 2.1 | 3174 |
| items-100k-mod | 100000 | 0 | 32.142 | 54.227 | 127.399 | 268.074 | 20.0 | 1731 |
| mixed-150k-mod | 150000 | 0 | 85.795 | 150.180 | 273.767 | 450.920 | 11.7 | 2161 |
| mobs-25k-mod | 25000 | 0 | 2640.145 | 2840.175 | 3003.427 | 3120.819 | 0.4 | 1985 |
| mobs-25k-native-mod | 25000 | 0 | 19.315 | 34.866 | 104.480 | 247.566 | 20.0 | 1916 |
| mobs-25k-native | 25000 | 0 | 322.042 | 387.809 | 464.404 | 559.912 | 3.1 | 1616 |
| spread-100k-mod | 100000 | 0 | 64.171 | 129.313 | 237.177 | 351.982 | 15.6 | 2112 |
| spread-100k | 100000 | 0 | 441.339 | 533.220 | 635.800 | 756.367 | 2.3 | 1956 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

