# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 81.594 | 99.700 | 185.216 | 242.728 | 12.3 | 870 |
| mobs-25k | 25000 | 0 | 399.305 | 441.985 | 476.073 | 523.929 | 2.5 | 1870 |
| mixed-150k | 150000 | 0 | 645.291 | 792.654 | 1065.014 | 1277.044 | 1.5 | 2982 |
| items-100k-mod | 100000 | 0 | 32.306 | 97.473 | 173.719 | 268.311 | 20.0 | 1476 |
| mixed-150k-mod | 150000 | 0 | 247.544 | 362.461 | 560.582 | 795.766 | 4.0 | 2035 |
| mobs-25k-mod | 25000 | 0 | 235.037 | 429.667 | 577.148 | 665.217 | 4.3 | 1463 |
| spread-100k-mod | 100000 | 0 | 270.022 | 478.235 | 615.255 | 820.549 | 3.7 | 2082 |
| spread-100k | 100000 | 0 | 631.453 | 702.188 | 968.064 | 1092.059 | 1.6 | 2500 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

