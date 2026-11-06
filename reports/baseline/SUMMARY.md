# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.165 | 81.585 | 150.340 | 184.403 | 15.3 | 1208 |
| mobs-25k | 25000 | 0 | 402.025 | 447.323 | 479.721 | 514.654 | 2.5 | 1294 |
| mixed-150k | 150000 | 0 | 558.036 | 696.284 | 936.862 | 1095.009 | 1.8 | 3079 |
| items-100k-mod | 100000 | 0 | 30.840 | 81.179 | 170.872 | 236.917 | 20.0 | 1353 |
| mixed-150k-mod | 150000 | 0 | 343.547 | 431.212 | 561.552 | 663.397 | 2.9 | 2478 |
| mobs-25k-mod | 25000 | 0 | 305.429 | 471.757 | 649.041 | 782.803 | 3.3 | 1453 |
| spread-100k-mod | 100000 | 0 | 341.975 | 463.333 | 590.409 | 701.224 | 2.9 | 3331 |
| spread-100k | 100000 | 0 | 428.632 | 490.972 | 701.494 | 752.104 | 2.3 | 2158 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

