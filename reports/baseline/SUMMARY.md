# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.794 | 85.952 | 162.876 | 206.104 | 14.3 | 1486 |
| mobs-25k | 25000 | 0 | 250.535 | 285.513 | 319.591 | 358.216 | 4.0 | 1081 |
| mixed-150k | 150000 | 0 | 504.820 | 663.289 | 943.416 | 1119.485 | 2.0 | 2990 |
| items-100k-mod | 100000 | 0 | 14.620 | 19.108 | 98.737 | 137.385 | 20.0 | 1147 |
| mixed-150k-mod | 150000 | 0 | 268.592 | 321.221 | 377.012 | 454.660 | 3.7 | 2222 |
| mobs-25k-mod | 25000 | 0 | 147.485 | 221.034 | 291.077 | 333.098 | 6.8 | 1541 |
| spread-100k-mod | 100000 | 0 | 157.354 | 232.705 | 305.077 | 389.741 | 6.4 | 1819 |
| spread-100k | 100000 | 0 | 427.960 | 486.498 | 697.428 | 835.256 | 2.3 | 2006 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

