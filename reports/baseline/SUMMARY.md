# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 73.540 | 91.371 | 162.580 | 205.135 | 13.6 | 1822 |
| mobs-25k | 25000 | 0 | 514.567 | 570.860 | 621.966 | 679.774 | 1.9 | 1077 |
| mixed-150k | 150000 | 0 | 633.919 | 758.188 | 1000.147 | 1109.382 | 1.6 | 1707 |
| items-100k-mod | 100000 | 0 | 37.178 | 97.625 | 177.181 | 217.013 | 20.0 | 1297 |
| mixed-150k-mod | 150000 | 0 | 291.725 | 388.365 | 601.638 | 682.051 | 3.4 | 1779 |
| mobs-25k-mod | 25000 | 0 | 287.958 | 460.711 | 556.079 | 635.833 | 3.5 | 1142 |
| spread-100k-mod | 100000 | 0 | 322.319 | 523.931 | 644.281 | 880.584 | 3.1 | 1922 |
| spread-100k | 100000 | 0 | 526.131 | 588.209 | 835.276 | 924.035 | 1.9 | 1982 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

