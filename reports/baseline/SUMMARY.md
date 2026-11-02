# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 74.546 | 96.091 | 178.197 | 246.996 | 13.4 | 1165 |
| mobs-25k | 25000 | 0 | 524.556 | 631.758 | 936.010 | 1043.050 | 1.9 | 1688 |
| mixed-150k | 150000 | 0 | 656.570 | 828.191 | 1156.448 | 1307.999 | 1.5 | 3191 |
| items-100k-mod | 100000 | 0 | 29.555 | 86.061 | 165.410 | 249.787 | 20.0 | 1257 |
| mixed-150k-mod | 150000 | 0 | 246.900 | 356.221 | 503.920 | 632.735 | 4.1 | 1873 |
| mobs-25k-mod | 25000 | 0 | 284.609 | 481.477 | 661.080 | 814.173 | 3.5 | 1450 |
| spread-100k-mod | 100000 | 0 | 263.053 | 398.825 | 488.265 | 619.221 | 3.8 | 2631 |
| spread-100k | 100000 | 0 | 439.767 | 513.002 | 761.176 | 884.331 | 2.3 | 2087 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

