# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 61.350 | 77.667 | 139.919 | 173.315 | 16.3 | 2275 |
| mobs-25k | 25000 | 0 | 360.775 | 401.992 | 446.185 | 478.208 | 2.8 | 1860 |
| mixed-150k | 150000 | 0 | 484.221 | 611.354 | 797.991 | 1318.610 | 2.1 | 3063 |
| items-100k-mod | 100000 | 0 | 33.074 | 57.525 | 158.084 | 220.714 | 20.0 | 1187 |
| mixed-150k-mod | 150000 | 0 | 182.589 | 331.784 | 391.770 | 409.819 | 5.5 | 2478 |
| mobs-25k-mod | 25000 | 0 | 169.225 | 314.175 | 391.342 | 436.663 | 5.9 | 1679 |
| spread-100k-mod | 100000 | 0 | 424.737 | 490.639 | 512.002 | 571.339 | 2.4 | 1919 |
| spread-100k | 100000 | 0 | 397.222 | 469.495 | 638.722 | 712.702 | 2.5 | 2105 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

