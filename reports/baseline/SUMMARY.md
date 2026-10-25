# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.074 | 78.689 | 146.515 | 188.831 | 15.6 | 1125 |
| mobs-25k | 25000 | 0 | 360.751 | 405.411 | 446.429 | 487.042 | 2.8 | 1352 |
| mixed-150k | 150000 | 0 | 452.419 | 557.137 | 775.998 | 884.481 | 2.2 | 2628 |
| items-100k-mod | 100000 | 0 | 15.157 | 19.022 | 81.654 | 104.161 | 20.0 | 1156 |
| mixed-150k-mod | 150000 | 0 | 274.307 | 332.369 | 396.353 | 437.976 | 3.6 | 2959 |
| mobs-25k-mod | 25000 | 0 | 152.947 | 284.342 | 356.892 | 428.760 | 6.5 | 1506 |
| spread-100k-mod | 100000 | 0 | 136.606 | 222.849 | 307.245 | 402.615 | 7.3 | 1857 |
| spread-100k | 100000 | 0 | 345.160 | 410.957 | 600.429 | 668.882 | 2.9 | 2100 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

