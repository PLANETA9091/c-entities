# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 84.091 | 98.756 | 181.775 | 229.083 | 11.9 | 2296 |
| mobs-25k | 25000 | 0 | 276.209 | 314.578 | 359.065 | 415.439 | 3.6 | 1294 |
| mixed-150k | 150000 | 0 | 490.149 | 618.047 | 906.157 | 1021.658 | 2.0 | 1653 |
| items-100k-mod | 100000 | 0 | 15.983 | 23.873 | 102.547 | 137.490 | 20.0 | 1293 |
| mixed-150k-mod | 150000 | 0 | 276.049 | 367.487 | 505.687 | 580.252 | 3.6 | 1651 |
| mobs-25k-mod | 25000 | 0 | 152.142 | 233.104 | 420.199 | 474.263 | 6.6 | 1814 |
| spread-100k-mod | 100000 | 0 | 148.920 | 258.604 | 405.113 | 528.276 | 6.7 | 2324 |
| spread-100k | 100000 | 0 | 418.608 | 477.977 | 699.711 | 728.213 | 2.4 | 2184 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

