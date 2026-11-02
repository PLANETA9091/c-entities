# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 76.125 | 91.105 | 168.718 | 217.895 | 13.1 | 2169 |
| mobs-25k | 25000 | 0 | 537.408 | 596.193 | 643.814 | 703.035 | 1.9 | 1428 |
| mixed-150k | 150000 | 0 | 681.735 | 870.295 | 1285.893 | 1449.489 | 1.5 | 2585 |
| items-100k-mod | 100000 | 0 | 34.547 | 91.968 | 170.869 | 273.766 | 20.0 | 1185 |
| mixed-150k-mod | 150000 | 0 | 275.756 | 391.103 | 602.295 | 722.632 | 3.6 | 1842 |
| mobs-25k-mod | 25000 | 0 | 309.931 | 555.816 | 702.698 | 965.621 | 3.2 | 1766 |
| spread-100k-mod | 100000 | 0 | 340.892 | 538.272 | 667.069 | 980.500 | 2.9 | 1986 |
| spread-100k | 100000 | 0 | 561.833 | 653.919 | 908.775 | 1096.672 | 1.8 | 2068 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

