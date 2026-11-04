# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.130 | 82.329 | 155.204 | 216.905 | 14.9 | 1403 |
| mobs-25k | 25000 | 0 | 401.817 | 446.209 | 493.878 | 520.652 | 2.5 | 1194 |
| mixed-150k | 150000 | 0 | 678.733 | 821.271 | 1164.310 | 1288.187 | 1.5 | 1784 |
| items-100k-mod | 100000 | 0 | 29.293 | 81.972 | 166.739 | 237.475 | 20.0 | 1397 |
| mixed-150k-mod | 150000 | 0 | 402.652 | 507.399 | 670.611 | 804.915 | 2.5 | 1962 |
| mobs-25k-mod | 25000 | 0 | 300.212 | 504.147 | 609.247 | 682.380 | 3.3 | 2057 |
| spread-100k-mod | 100000 | 0 | 287.631 | 391.305 | 499.406 | 628.466 | 3.5 | 2127 |
| spread-100k | 100000 | 0 | 321.598 | 391.877 | 545.442 | 620.468 | 3.1 | 2143 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

