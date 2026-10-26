# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 58.170 | 72.495 | 128.398 | 172.776 | 17.2 | 750 |
| mobs-25k | 25000 | 0 | 377.679 | 428.021 | 465.096 | 482.160 | 2.6 | 1077 |
| mixed-150k | 150000 | 0 | 497.396 | 596.298 | 833.679 | 906.323 | 2.0 | 2861 |
| items-100k-mod | 100000 | 0 | 9.746 | 13.454 | 65.746 | 95.934 | 20.0 | 1155 |
| mixed-150k-mod | 150000 | 0 | 288.690 | 354.056 | 405.862 | 458.543 | 3.5 | 2457 |
| mobs-25k-mod | 25000 | 0 | 197.086 | 288.405 | 365.743 | 411.564 | 5.1 | 1304 |
| spread-100k-mod | 100000 | 0 | 139.348 | 205.766 | 280.330 | 352.007 | 7.2 | 1888 |
| spread-100k | 100000 | 0 | 360.077 | 426.885 | 628.546 | 678.325 | 2.8 | 1789 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

