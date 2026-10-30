# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.241 | 87.075 | 157.341 | 198.146 | 14.7 | 949 |
| mobs-25k | 25000 | 0 | 328.363 | 371.301 | 409.957 | 487.893 | 3.0 | 1489 |
| mixed-150k | 150000 | 0 | 415.201 | 498.918 | 674.834 | 718.139 | 2.4 | 2306 |
| items-100k-mod | 100000 | 0 | 33.809 | 91.674 | 150.126 | 225.567 | 20.0 | 1386 |
| mixed-150k-mod | 150000 | 0 | 229.415 | 324.263 | 403.783 | 530.871 | 4.4 | 1750 |
| mobs-25k-mod | 25000 | 0 | 192.737 | 320.436 | 518.027 | 576.591 | 5.2 | 1339 |
| spread-100k-mod | 100000 | 0 | 306.620 | 444.755 | 573.374 | 754.600 | 3.3 | 1977 |
| spread-100k | 100000 | 0 | 633.365 | 711.488 | 992.132 | 1072.121 | 1.6 | 2825 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

