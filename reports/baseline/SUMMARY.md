# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 76.779 | 94.526 | 181.523 | 225.752 | 13.0 | 964 |
| mobs-25k | 25000 | 0 | 541.097 | 596.295 | 661.652 | 688.869 | 1.8 | 1807 |
| mixed-150k | 150000 | 0 | 434.362 | 521.355 | 746.235 | 832.220 | 2.3 | 2501 |
| items-100k-mod | 100000 | 0 | 37.091 | 98.889 | 170.166 | 221.140 | 20.0 | 1323 |
| mixed-150k-mod | 150000 | 0 | 225.023 | 311.013 | 436.268 | 515.415 | 4.4 | 1677 |
| mobs-25k-mod | 25000 | 0 | 279.540 | 515.031 | 693.150 | 761.489 | 3.6 | 1781 |
| spread-100k-mod | 100000 | 0 | 297.943 | 466.374 | 623.468 | 783.074 | 3.4 | 1846 |
| spread-100k | 100000 | 0 | 539.321 | 604.775 | 867.794 | 1003.259 | 1.9 | 2074 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

