# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 58.090 | 71.034 | 134.406 | 172.277 | 17.2 | 1801 |
| mobs-25k | 25000 | 0 | 306.064 | 344.704 | 418.817 | 616.036 | 3.3 | 1300 |
| mixed-150k | 150000 | 0 | 669.057 | 826.633 | 1220.213 | 1336.577 | 1.5 | 3460 |
| items-100k-mod | 100000 | 0 | 19.563 | 44.922 | 107.137 | 173.536 | 20.0 | 1260 |
| mixed-150k-mod | 150000 | 0 | 269.588 | 378.518 | 554.702 | 643.671 | 3.7 | 1949 |
| mobs-25k-mod | 25000 | 0 | 192.919 | 290.130 | 466.434 | 513.087 | 5.2 | 1471 |
| spread-100k-mod | 100000 | 0 | 322.647 | 522.476 | 641.292 | 849.891 | 3.1 | 2067 |
| spread-100k | 100000 | 0 | 514.234 | 597.072 | 825.525 | 935.104 | 1.9 | 2118 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

