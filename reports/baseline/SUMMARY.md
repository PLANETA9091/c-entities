# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 57.299 | 98.090 | 150.740 | 180.700 | 17.5 | 1210 |
| mobs-25k | 25000 | 0 | 731.278 | 812.666 | 885.891 | 1098.032 | 1.4 | 1674 |
| mixed-150k | 150000 | 0 | 385.135 | 524.283 | 760.396 | 995.588 | 2.6 | 2591 |
| items-100k-mod | 100000 | 0 | 11.581 | 20.513 | 87.335 | 308.078 | 20.0 | 1684 |
| mixed-150k-mod | 150000 | 0 | 61.652 | 101.519 | 220.073 | 301.252 | 16.2 | 2319 |
| mobs-25k-mod | 25000 | 0 | 355.431 | 2709.054 | 4170.979 | 4376.274 | 2.8 | 1659 |
| mobs-25k-native-mod | 25000 | 0 | 21.400 | 37.419 | 107.656 | 214.502 | 20.0 | 1672 |
| mobs-25k-native | 25000 | 0 | 560.213 | 637.595 | 715.252 | 869.072 | 1.8 | 1428 |
| spread-100k-mod | 100000 | 0 | 57.091 | 99.379 | 227.142 | 287.727 | 17.5 | 2481 |
| spread-100k | 100000 | 0 | 545.296 | 660.703 | 745.731 | 894.576 | 1.8 | 1872 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

