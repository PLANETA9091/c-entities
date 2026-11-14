# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.732 | 105.274 | 155.679 | 196.328 | 15.2 | 1065 |
| mobs-25k | 25000 | 0 | 570.830 | 648.383 | 724.034 | 852.363 | 1.8 | 1830 |
| mixed-150k | 150000 | 0 | 606.825 | 788.088 | 1126.982 | 1301.234 | 1.6 | 1796 |
| items-100k-mod | 100000 | 0 | 33.287 | 53.225 | 125.574 | 267.482 | 20.0 | 1602 |
| mixed-150k-mod | 150000 | 0 | 108.842 | 200.003 | 365.071 | 655.870 | 9.2 | 2062 |
| mobs-25k-mod | 25000 | 0 | 2629.074 | 2852.336 | 3022.772 | 3161.236 | 0.4 | 1796 |
| mobs-25k-native-mod | 25000 | 0 | 20.467 | 35.296 | 105.510 | 147.255 | 20.0 | 1370 |
| mobs-25k-native | 25000 | 0 | 496.892 | 573.876 | 641.938 | 776.036 | 2.0 | 1358 |
| spread-100k-mod | 100000 | 0 | 68.368 | 139.176 | 236.306 | 343.279 | 14.6 | 2030 |
| spread-100k | 100000 | 0 | 535.051 | 625.068 | 722.072 | 807.293 | 1.9 | 2386 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

