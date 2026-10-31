# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.700 | 83.044 | 157.796 | 221.472 | 14.8 | 1640 |
| mobs-25k | 25000 | 0 | 523.140 | 583.013 | 638.779 | 673.456 | 1.9 | 908 |
| mixed-150k | 150000 | 0 | 893.629 | 1247.092 | 1816.702 | 2381.000 | 1.1 | 3174 |
| items-100k-mod | 100000 | 0 | 34.405 | 94.009 | 164.342 | 236.834 | 20.0 | 1154 |
| mixed-150k-mod | 150000 | 0 | 320.478 | 430.649 | 590.745 | 776.879 | 3.1 | 1712 |
| mobs-25k-mod | 25000 | 0 | 269.550 | 528.273 | 658.494 | 719.135 | 3.7 | 1714 |
| spread-100k-mod | 100000 | 0 | 264.672 | 397.292 | 460.263 | 705.339 | 3.8 | 1969 |
| spread-100k | 100000 | 0 | 483.064 | 554.739 | 809.654 | 918.008 | 2.1 | 2155 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

