# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.607 | 77.756 | 144.175 | 180.230 | 16.0 | 1820 |
| mobs-25k | 25000 | 0 | 420.848 | 477.868 | 533.602 | 660.720 | 2.4 | 1373 |
| mixed-150k | 150000 | 0 | 512.587 | 649.081 | 992.365 | 1079.317 | 2.0 | 1677 |
| items-100k-mod | 100000 | 0 | 11.560 | 16.032 | 100.227 | 125.941 | 20.0 | 1126 |
| mixed-150k-mod | 150000 | 0 | 293.144 | 399.064 | 580.096 | 627.993 | 3.4 | 1517 |
| mobs-25k-mod | 25000 | 0 | 235.003 | 447.020 | 668.879 | 743.614 | 4.3 | 1651 |
| spread-100k-mod | 100000 | 0 | 155.073 | 265.007 | 423.274 | 496.737 | 6.4 | 1936 |
| spread-100k | 100000 | 0 | 413.959 | 475.011 | 689.206 | 707.954 | 2.4 | 2209 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

