# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.193 | 86.743 | 160.286 | 223.616 | 14.7 | 2103 |
| mobs-25k | 25000 | 0 | 334.432 | 380.779 | 412.021 | 469.345 | 3.0 | 1554 |
| mixed-150k | 150000 | 0 | 432.482 | 516.920 | 734.459 | 931.846 | 2.3 | 2212 |
| items-100k-mod | 100000 | 0 | 34.149 | 91.088 | 150.291 | 182.344 | 20.0 | 1178 |
| mixed-150k-mod | 150000 | 0 | 343.239 | 481.721 | 627.732 | 721.716 | 2.9 | 2341 |
| mobs-25k-mod | 25000 | 0 | 198.653 | 320.036 | 509.106 | 627.467 | 5.0 | 1422 |
| spread-100k-mod | 100000 | 0 | 198.926 | 348.248 | 544.304 | 827.382 | 5.0 | 2094 |
| spread-100k | 100000 | 0 | 455.933 | 526.587 | 756.263 | 850.741 | 2.2 | 1998 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

