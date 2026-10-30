# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.401 | 84.184 | 153.105 | 192.611 | 15.1 | 973 |
| mobs-25k | 25000 | 0 | 515.167 | 570.409 | 627.719 | 660.838 | 1.9 | 926 |
| mixed-150k | 150000 | 0 | 722.521 | 863.044 | 1279.879 | 1405.278 | 1.4 | 2257 |
| items-100k-mod | 100000 | 0 | 29.750 | 83.988 | 153.224 | 202.087 | 20.0 | 1471 |
| mixed-150k-mod | 150000 | 0 | 299.611 | 424.823 | 640.683 | 803.923 | 3.3 | 1741 |
| mobs-25k-mod | 25000 | 0 | 285.016 | 450.946 | 591.361 | 665.179 | 3.5 | 1372 |
| spread-100k-mod | 100000 | 0 | 254.014 | 395.893 | 500.800 | 699.227 | 3.9 | 2796 |
| spread-100k | 100000 | 0 | 429.518 | 498.310 | 683.320 | 824.271 | 2.3 | 2068 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

