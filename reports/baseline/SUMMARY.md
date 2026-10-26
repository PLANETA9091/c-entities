# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.393 | 83.262 | 146.596 | 189.389 | 15.3 | 971 |
| mobs-25k | 25000 | 0 | 393.669 | 440.099 | 491.569 | 524.007 | 2.5 | 899 |
| mixed-150k | 150000 | 0 | 451.379 | 552.232 | 768.068 | 834.109 | 2.2 | 2921 |
| items-100k-mod | 100000 | 0 | 13.795 | 17.614 | 84.575 | 104.501 | 20.0 | 1055 |
| mixed-150k-mod | 150000 | 0 | 243.508 | 294.434 | 344.526 | 436.647 | 4.1 | 1566 |
| mobs-25k-mod | 25000 | 0 | 206.093 | 299.484 | 396.148 | 477.747 | 4.9 | 1190 |
| spread-100k-mod | 100000 | 0 | 145.089 | 217.110 | 303.287 | 355.332 | 6.9 | 2445 |
| spread-100k | 100000 | 0 | 400.163 | 462.211 | 655.415 | 699.220 | 2.5 | 2212 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

