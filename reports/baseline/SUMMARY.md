# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.034 | 122.721 | 188.090 | 230.763 | 14.5 | 1916 |
| mobs-25k | 25000 | 0 | 519.810 | 592.230 | 681.055 | 972.052 | 1.9 | 1865 |
| mixed-150k | 150000 | 0 | 416.888 | 561.921 | 792.550 | 926.810 | 2.4 | 2866 |
| items-100k-mod | 100000 | 0 | 30.265 | 46.347 | 112.489 | 235.875 | 20.0 | 1536 |
| mixed-150k-mod | 150000 | 0 | 569.554 | 692.253 | 885.788 | 1004.379 | 1.8 | 2875 |
| mobs-25k-mod | 25000 | 0 | 668.900 | 929.400 | 1118.833 | 1209.688 | 1.5 | 2061 |
| spread-100k-mod | 100000 | 0 | 627.054 | 896.234 | 1224.807 | 1463.078 | 1.6 | 3191 |
| spread-100k | 100000 | 0 | 469.893 | 669.044 | 956.526 | 1370.331 | 2.1 | 2331 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

