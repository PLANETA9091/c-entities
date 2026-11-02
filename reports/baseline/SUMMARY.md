# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 61.627 | 75.897 | 140.125 | 176.026 | 16.2 | 845 |
| mobs-25k | 25000 | 0 | 328.681 | 384.308 | 451.065 | 488.743 | 3.0 | 1936 |
| mixed-150k | 150000 | 0 | 708.867 | 922.404 | 1285.282 | 1476.232 | 1.4 | 2592 |
| items-100k-mod | 100000 | 0 | 20.423 | 46.831 | 106.867 | 121.577 | 20.0 | 1323 |
| mixed-150k-mod | 150000 | 0 | 260.511 | 362.943 | 485.205 | 601.819 | 3.8 | 1851 |
| mobs-25k-mod | 25000 | 0 | 204.732 | 325.198 | 457.625 | 523.098 | 4.9 | 1393 |
| spread-100k-mod | 100000 | 0 | 221.154 | 347.264 | 468.442 | 570.348 | 4.5 | 2185 |
| spread-100k | 100000 | 0 | 347.658 | 413.438 | 587.869 | 690.906 | 2.9 | 1974 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

