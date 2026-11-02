# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.175 | 86.543 | 158.856 | 217.557 | 14.5 | 1608 |
| mobs-25k | 25000 | 0 | 308.184 | 345.768 | 374.020 | 464.974 | 3.2 | 1339 |
| mixed-150k | 150000 | 0 | 401.893 | 483.667 | 658.386 | 702.249 | 2.5 | 2114 |
| items-100k-mod | 100000 | 0 | 28.982 | 91.124 | 163.074 | 233.576 | 20.0 | 1500 |
| mixed-150k-mod | 150000 | 0 | 192.301 | 271.022 | 377.813 | 512.285 | 5.2 | 1812 |
| mobs-25k-mod | 25000 | 0 | 187.864 | 291.583 | 456.848 | 488.825 | 5.3 | 1833 |
| spread-100k-mod | 100000 | 0 | 303.510 | 481.003 | 611.321 | 894.731 | 3.3 | 2002 |
| spread-100k | 100000 | 0 | 542.650 | 609.123 | 873.042 | 928.419 | 1.8 | 2093 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

