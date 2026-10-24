# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.897 | 79.015 | 142.366 | 173.807 | 15.7 | 1479 |
| mobs-25k | 25000 | 0 | 374.456 | 432.827 | 489.413 | 539.378 | 2.7 | 940 |
| mixed-150k | 150000 | 0 | 511.951 | 641.195 | 897.686 | 1180.785 | 2.0 | 3212 |
| items-100k-mod | 100000 | 0 | 47.366 | 70.361 | 203.349 | 235.422 | 20.0 | 1344 |
| mixed-150k-mod | 150000 | 0 | 355.365 | 429.804 | 469.749 | 540.022 | 2.8 | 1867 |
| mobs-25k-mod | 25000 | 0 | 182.973 | 342.049 | 412.586 | 498.881 | 5.5 | 1166 |
| spread-100k-mod | 100000 | 0 | 165.587 | 259.567 | 347.889 | 471.118 | 6.0 | 2042 |
| spread-100k | 100000 | 0 | 391.786 | 454.029 | 649.793 | 668.588 | 2.6 | 2140 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

