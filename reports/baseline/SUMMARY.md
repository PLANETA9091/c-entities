# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.427 | 118.793 | 178.142 | 257.072 | 14.0 | 1893 |
| mobs-25k | 25000 | 0 | 571.986 | 641.835 | 752.590 | 806.586 | 1.7 | 1771 |
| mixed-150k | 150000 | 0 | 477.420 | 652.256 | 852.381 | 977.976 | 2.1 | 3051 |
| items-100k-mod | 100000 | 0 | 28.011 | 43.828 | 109.377 | 186.797 | 20.0 | 1430 |
| mixed-150k-mod | 150000 | 0 | 723.362 | 852.424 | 1087.581 | 1190.214 | 1.4 | 2193 |
| mobs-25k-mod | 25000 | 0 | 679.448 | 878.613 | 1113.111 | 1205.799 | 1.5 | 1758 |
| spread-100k-mod | 100000 | 0 | 398.460 | 537.967 | 707.830 | 864.368 | 2.5 | 2378 |
| spread-100k | 100000 | 0 | 327.180 | 421.097 | 583.553 | 716.958 | 3.1 | 2047 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

