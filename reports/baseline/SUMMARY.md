# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 60.270 | 73.542 | 133.992 | 187.809 | 16.6 | 1329 |
| mobs-25k | 25000 | 0 | 270.036 | 309.796 | 361.693 | 385.747 | 3.7 | 1645 |
| mixed-150k | 150000 | 0 | 514.669 | 640.921 | 917.818 | 1126.787 | 1.9 | 2918 |
| items-100k-mod | 100000 | 0 | 12.459 | 16.283 | 80.021 | 119.898 | 20.0 | 1332 |
| mixed-150k-mod | 150000 | 0 | 278.689 | 334.370 | 382.699 | 468.168 | 3.6 | 1634 |
| mobs-25k-mod | 25000 | 0 | 128.775 | 203.074 | 286.405 | 339.253 | 7.8 | 1857 |
| spread-100k-mod | 100000 | 0 | 127.551 | 196.027 | 274.076 | 371.505 | 7.8 | 2106 |
| spread-100k | 100000 | 0 | 376.305 | 435.105 | 618.316 | 684.446 | 2.7 | 1972 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

