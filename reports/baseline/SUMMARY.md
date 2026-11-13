# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.996 | 113.929 | 174.321 | 304.210 | 14.7 | 1805 |
| mobs-25k | 25000 | 0 | 389.758 | 464.751 | 584.441 | 655.850 | 2.6 | 1820 |
| mixed-150k | 150000 | 0 | 610.568 | 826.329 | 1202.407 | 1354.662 | 1.6 | 2344 |
| items-100k-mod | 100000 | 0 | 23.578 | 35.849 | 103.808 | 216.864 | 20.0 | 1552 |
| mixed-150k-mod | 150000 | 0 | 170.131 | 314.534 | 472.132 | 578.154 | 5.9 | 2252 |
| mobs-25k-mod | 25000 | 0 | 2712.236 | 2875.846 | 3025.522 | 3114.843 | 0.4 | 1518 |
| mobs-25k-native-mod | 25000 | 0 | 44.611 | 84.603 | 196.484 | 214.302 | 20.0 | 1502 |
| mobs-25k-native | 25000 | 0 | 578.961 | 654.894 | 738.609 | 850.795 | 1.7 | 1854 |
| spread-100k-mod | 100000 | 0 | 206.266 | 281.706 | 389.262 | 436.044 | 4.8 | 2030 |
| spread-100k | 100000 | 0 | 395.021 | 478.358 | 588.475 | 650.669 | 2.5 | 1931 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

