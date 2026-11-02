# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 75.474 | 97.561 | 173.163 | 214.842 | 13.2 | 1321 |
| mobs-25k | 25000 | 0 | 530.107 | 586.302 | 627.567 | 682.848 | 1.9 | 2381 |
| mixed-150k | 150000 | 0 | 678.261 | 813.220 | 1146.533 | 1265.677 | 1.5 | 2250 |
| items-100k-mod | 100000 | 0 | 32.284 | 79.556 | 169.378 | 219.777 | 20.0 | 1563 |
| mixed-150k-mod | 150000 | 0 | 251.059 | 353.481 | 473.345 | 567.806 | 4.0 | 1856 |
| mobs-25k-mod | 25000 | 0 | 296.292 | 473.719 | 609.912 | 777.646 | 3.4 | 1904 |
| spread-100k-mod | 100000 | 0 | 257.024 | 363.781 | 503.807 | 659.466 | 3.9 | 2285 |
| spread-100k | 100000 | 0 | 366.798 | 450.471 | 620.327 | 746.242 | 2.7 | 1941 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

