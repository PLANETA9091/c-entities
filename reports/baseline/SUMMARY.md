# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.027 | 85.509 | 153.088 | 198.408 | 14.7 | 1733 |
| mobs-25k | 25000 | 0 | 262.142 | 307.179 | 346.210 | 377.914 | 3.8 | 1492 |
| mixed-150k | 150000 | 0 | 413.245 | 489.602 | 678.824 | 784.360 | 2.4 | 2522 |
| items-100k-mod | 100000 | 0 | 14.277 | 21.000 | 93.338 | 120.442 | 20.0 | 1363 |
| mixed-150k-mod | 150000 | 0 | 253.336 | 334.402 | 438.403 | 500.428 | 3.9 | 1519 |
| mobs-25k-mod | 25000 | 0 | 161.678 | 270.227 | 451.437 | 585.999 | 6.2 | 1501 |
| spread-100k-mod | 100000 | 0 | 158.920 | 271.424 | 376.760 | 540.403 | 6.3 | 2672 |
| spread-100k | 100000 | 0 | 424.129 | 493.025 | 673.887 | 729.713 | 2.4 | 2131 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

