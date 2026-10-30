# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.215 | 84.287 | 147.703 | 217.502 | 14.7 | 1019 |
| mobs-25k | 25000 | 0 | 553.264 | 626.933 | 681.115 | 716.863 | 1.8 | 940 |
| mixed-150k | 150000 | 0 | 621.804 | 784.544 | 1082.103 | 1294.674 | 1.6 | 3104 |
| items-100k-mod | 100000 | 0 | 38.820 | 80.887 | 135.270 | 168.371 | 20.0 | 1489 |
| mixed-150k-mod | 150000 | 0 | 294.650 | 411.111 | 598.050 | 685.619 | 3.4 | 1709 |
| mobs-25k-mod | 25000 | 0 | 265.496 | 428.947 | 576.223 | 654.593 | 3.8 | 1818 |
| spread-100k-mod | 100000 | 0 | 332.664 | 481.157 | 624.230 | 784.087 | 3.0 | 1906 |
| spread-100k | 100000 | 0 | 519.535 | 587.031 | 822.867 | 931.678 | 1.9 | 2155 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

