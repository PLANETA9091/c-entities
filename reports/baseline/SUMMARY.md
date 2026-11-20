# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.011 | 106.160 | 161.884 | 203.211 | 15.1 | 943 |
| mobs-25k | 25000 | 0 | 309.862 | 383.090 | 473.012 | 557.718 | 3.2 | 1709 |
| mixed-150k | 150000 | 0 | 490.088 | 658.137 | 930.592 | 1048.174 | 2.0 | 3092 |
| items-100k-mod | 100000 | 0 | 34.221 | 76.889 | 180.389 | 339.239 | 20.0 | 1234 |
| mixed-150k-mod | 150000 | 0 | 115.108 | 234.094 | 449.391 | 803.381 | 8.7 | 2411 |
| mobs-25k-mod | 25000 | 0 | 1993.669 | 2227.067 | 2342.951 | 2483.849 | 0.5 | 1805 |
| mobs-25k-native-mod | 25000 | 0 | 31.689 | 79.406 | 148.604 | 303.970 | 20.0 | 1500 |
| mobs-25k-native | 25000 | 0 | 408.278 | 479.676 | 637.604 | 738.858 | 2.4 | 1722 |
| spread-100k-mod | 100000 | 0 | 83.438 | 152.476 | 290.992 | 394.786 | 12.0 | 1958 |
| spread-100k | 100000 | 0 | 548.358 | 638.668 | 761.125 | 831.133 | 1.8 | 1864 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

