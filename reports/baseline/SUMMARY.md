# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 50.720 | 64.902 | 114.800 | 209.575 | 19.7 | 1893 |
| mobs-25k | 25000 | 0 | 399.960 | 448.142 | 485.513 | 524.572 | 2.5 | 1003 |
| mixed-150k | 150000 | 0 | 662.879 | 822.277 | 1144.201 | 1418.173 | 1.5 | 3270 |
| items-100k-mod | 100000 | 0 | 23.159 | 58.210 | 119.887 | 156.849 | 20.0 | 1123 |
| mixed-150k-mod | 150000 | 0 | 293.907 | 415.119 | 638.316 | 721.649 | 3.4 | 1828 |
| mobs-25k-mod | 25000 | 0 | 245.520 | 380.080 | 511.813 | 663.444 | 4.1 | 1175 |
| spread-100k-mod | 100000 | 0 | 298.814 | 447.646 | 572.227 | 798.427 | 3.3 | 2497 |
| spread-100k | 100000 | 0 | 508.664 | 580.280 | 791.646 | 881.922 | 2.0 | 2262 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

