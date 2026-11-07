# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.568 | 121.317 | 180.436 | 237.998 | 14.2 | 2258 |
| mobs-25k | 25000 | 0 | 343.655 | 407.532 | 493.381 | 594.192 | 2.9 | 1694 |
| mixed-150k | 150000 | 0 | 586.649 | 834.571 | 1111.656 | 1520.228 | 1.7 | 1754 |
| items-100k-mod | 100000 | 0 | 28.309 | 42.517 | 127.385 | 335.550 | 20.0 | 1580 |
| mixed-150k-mod | 150000 | 0 | 821.613 | 953.444 | 1191.463 | 1289.269 | 1.2 | 2037 |
| mobs-25k-mod | 25000 | 0 | 433.201 | 644.210 | 777.897 | 816.931 | 2.3 | 1990 |
| spread-100k-mod | 100000 | 0 | 610.716 | 714.639 | 952.245 | 1186.708 | 1.6 | 2432 |
| spread-100k | 100000 | 0 | 451.623 | 542.730 | 733.873 | 826.808 | 2.2 | 2284 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

