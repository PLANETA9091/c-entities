# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 78.331 | 95.807 | 177.358 | 247.741 | 12.8 | 1812 |
| mobs-25k | 25000 | 0 | 531.977 | 593.225 | 645.222 | 686.093 | 1.9 | 1541 |
| mixed-150k | 150000 | 0 | 692.365 | 885.296 | 1207.559 | 1635.201 | 1.4 | 2457 |
| items-100k-mod | 100000 | 0 | 36.857 | 89.162 | 160.900 | 209.614 | 20.0 | 1434 |
| mixed-150k-mod | 150000 | 0 | 282.542 | 386.222 | 522.594 | 621.184 | 3.5 | 1657 |
| mobs-25k-mod | 25000 | 0 | 283.916 | 545.559 | 688.066 | 811.865 | 3.5 | 1387 |
| spread-100k-mod | 100000 | 0 | 258.380 | 433.746 | 537.556 | 743.000 | 3.9 | 2036 |
| spread-100k | 100000 | 0 | 591.254 | 665.058 | 871.614 | 960.744 | 1.7 | 2068 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

