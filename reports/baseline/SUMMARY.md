# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 75.085 | 90.033 | 167.919 | 218.844 | 13.3 | 1115 |
| mobs-25k | 25000 | 0 | 496.174 | 552.777 | 604.438 | 625.972 | 2.0 | 995 |
| mixed-150k | 150000 | 0 | 463.398 | 606.459 | 821.516 | 1057.098 | 2.2 | 2879 |
| items-100k-mod | 100000 | 0 | 29.969 | 85.841 | 179.585 | 232.289 | 20.0 | 1288 |
| mixed-150k-mod | 150000 | 0 | 212.366 | 312.004 | 422.177 | 535.362 | 4.7 | 2022 |
| mobs-25k-mod | 25000 | 0 | 291.223 | 480.460 | 676.905 | 722.354 | 3.4 | 1527 |
| spread-100k-mod | 100000 | 0 | 232.263 | 346.221 | 463.881 | 571.407 | 4.3 | 2332 |
| spread-100k | 100000 | 0 | 406.138 | 471.215 | 642.125 | 771.233 | 2.5 | 2189 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

