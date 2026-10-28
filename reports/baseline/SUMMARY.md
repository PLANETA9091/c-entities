# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 86.263 | 104.051 | 185.974 | 237.239 | 11.6 | 2397 |
| mobs-25k | 25000 | 0 | 508.819 | 574.768 | 625.391 | 644.851 | 2.0 | 1732 |
| mixed-150k | 150000 | 0 | 421.808 | 514.435 | 672.483 | 793.086 | 2.4 | 2112 |
| items-100k-mod | 100000 | 0 | 38.707 | 108.368 | 187.535 | 209.044 | 20.0 | 1584 |
| mixed-150k-mod | 150000 | 0 | 324.169 | 444.118 | 590.521 | 682.012 | 3.1 | 2133 |
| mobs-25k-mod | 25000 | 0 | 243.553 | 435.298 | 678.145 | 797.547 | 4.1 | 2552 |
| spread-100k-mod | 100000 | 0 | 189.308 | 326.534 | 481.848 | 624.389 | 5.3 | 2488 |
| spread-100k | 100000 | 0 | 499.564 | 569.518 | 776.440 | 831.493 | 2.0 | 2141 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

