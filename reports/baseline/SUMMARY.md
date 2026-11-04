# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.648 | 87.343 | 156.891 | 228.017 | 14.2 | 1078 |
| mobs-25k | 25000 | 0 | 410.087 | 461.742 | 513.550 | 585.265 | 2.4 | 1662 |
| mixed-150k | 150000 | 0 | 668.959 | 799.465 | 1113.611 | 1285.974 | 1.5 | 3338 |
| items-100k-mod | 100000 | 0 | 32.602 | 95.233 | 159.166 | 257.392 | 20.0 | 1346 |
| mixed-150k-mod | 150000 | 0 | 401.916 | 505.193 | 707.330 | 783.328 | 2.5 | 1947 |
| mobs-25k-mod | 25000 | 0 | 291.813 | 492.409 | 608.381 | 696.184 | 3.4 | 1316 |
| spread-100k-mod | 100000 | 0 | 462.217 | 613.378 | 759.903 | 1079.996 | 2.2 | 2279 |
| spread-100k | 100000 | 0 | 549.721 | 646.068 | 921.512 | 1002.084 | 1.8 | 1999 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

