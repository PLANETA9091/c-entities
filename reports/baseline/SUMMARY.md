# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 54.706 | 70.375 | 124.865 | 180.443 | 18.3 | 1930 |
| mobs-25k | 25000 | 0 | 327.583 | 374.001 | 434.025 | 464.886 | 3.1 | 1555 |
| mixed-150k | 150000 | 0 | 705.380 | 871.136 | 1270.912 | 1387.540 | 1.4 | 1821 |
| items-100k-mod | 100000 | 0 | 19.173 | 46.357 | 103.063 | 178.076 | 20.0 | 1722 |
| mixed-150k-mod | 150000 | 0 | 250.617 | 361.572 | 525.766 | 624.177 | 4.0 | 1923 |
| mobs-25k-mod | 25000 | 0 | 192.170 | 312.728 | 450.779 | 574.476 | 5.2 | 1719 |
| spread-100k-mod | 100000 | 0 | 248.742 | 390.459 | 528.915 | 709.398 | 4.0 | 2211 |
| spread-100k | 100000 | 0 | 497.309 | 563.704 | 799.406 | 868.930 | 2.0 | 1900 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

