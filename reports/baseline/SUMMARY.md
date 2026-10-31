# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.990 | 79.078 | 141.999 | 207.826 | 15.9 | 1543 |
| mobs-25k | 25000 | 0 | 546.458 | 602.767 | 653.333 | 682.106 | 1.8 | 1245 |
| mixed-150k | 150000 | 0 | 543.479 | 670.356 | 877.092 | 1155.897 | 1.8 | 2875 |
| items-100k-mod | 100000 | 0 | 28.596 | 72.003 | 152.485 | 204.924 | 20.0 | 1221 |
| mixed-150k-mod | 150000 | 0 | 246.067 | 372.055 | 591.638 | 694.530 | 4.1 | 1948 |
| mobs-25k-mod | 25000 | 0 | 273.434 | 514.263 | 688.043 | 782.592 | 3.7 | 1575 |
| spread-100k-mod | 100000 | 0 | 261.290 | 402.586 | 516.498 | 738.817 | 3.8 | 2851 |
| spread-100k | 100000 | 0 | 381.141 | 449.500 | 637.376 | 915.664 | 2.6 | 2008 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

