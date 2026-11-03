# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 58.885 | 77.096 | 144.344 | 213.111 | 17.0 | 933 |
| mobs-25k | 25000 | 0 | 495.016 | 549.795 | 590.905 | 624.672 | 2.0 | 871 |
| mixed-150k | 150000 | 0 | 456.091 | 556.340 | 784.565 | 865.664 | 2.2 | 2712 |
| items-100k-mod | 100000 | 0 | 19.892 | 48.588 | 112.434 | 186.940 | 20.0 | 1462 |
| mixed-150k-mod | 150000 | 0 | 211.293 | 304.681 | 423.155 | 537.631 | 4.7 | 2199 |
| mobs-25k-mod | 25000 | 0 | 241.841 | 381.164 | 489.077 | 527.145 | 4.1 | 1418 |
| spread-100k-mod | 100000 | 0 | 228.281 | 357.791 | 504.116 | 578.630 | 4.4 | 2649 |
| spread-100k | 100000 | 0 | 346.804 | 413.057 | 555.179 | 622.107 | 2.9 | 2258 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

