# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.242 | 85.413 | 157.247 | 200.249 | 14.0 | 2161 |
| mobs-25k | 25000 | 0 | 366.387 | 414.760 | 474.141 | 496.799 | 2.7 | 1902 |
| mixed-150k | 150000 | 0 | 566.795 | 738.564 | 1113.396 | 1375.801 | 1.8 | 2747 |
| items-100k-mod | 100000 | 0 | 52.445 | 77.224 | 185.604 | 250.099 | 19.1 | 1328 |
| mixed-150k-mod | 150000 | 0 | 375.657 | 465.739 | 531.043 | 590.384 | 2.7 | 1766 |
| mobs-25k-mod | 25000 | 0 | 143.001 | 281.188 | 359.579 | 395.373 | 7.0 | 1204 |
| spread-100k-mod | 100000 | 0 | 166.959 | 267.973 | 363.725 | 475.969 | 6.0 | 1818 |
| spread-100k | 100000 | 0 | 406.507 | 466.793 | 667.314 | 855.051 | 2.5 | 2036 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

