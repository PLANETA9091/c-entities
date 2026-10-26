# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 59.596 | 74.963 | 136.085 | 194.978 | 16.8 | 1249 |
| mobs-25k | 25000 | 0 | 373.906 | 421.480 | 467.087 | 509.862 | 2.7 | 1146 |
| mixed-150k | 150000 | 0 | 360.762 | 443.476 | 612.943 | 862.034 | 2.8 | 2300 |
| items-100k-mod | 100000 | 0 | 12.729 | 16.391 | 75.303 | 93.403 | 20.0 | 1154 |
| mixed-150k-mod | 150000 | 0 | 211.521 | 262.513 | 313.308 | 509.393 | 4.7 | 2587 |
| mobs-25k-mod | 25000 | 0 | 158.261 | 261.242 | 337.069 | 380.808 | 6.3 | 1326 |
| spread-100k-mod | 100000 | 0 | 151.666 | 231.511 | 314.945 | 385.813 | 6.6 | 2614 |
| spread-100k | 100000 | 0 | 413.392 | 475.501 | 716.274 | 754.037 | 2.4 | 2216 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

