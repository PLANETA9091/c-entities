# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.536 | 89.776 | 168.767 | 225.481 | 13.8 | 1859 |
| mobs-25k | 25000 | 0 | 403.114 | 452.814 | 493.450 | 519.778 | 2.5 | 1846 |
| mixed-150k | 150000 | 0 | 665.439 | 837.278 | 1122.995 | 1284.091 | 1.5 | 3159 |
| items-100k-mod | 100000 | 0 | 31.716 | 90.345 | 169.777 | 231.944 | 20.0 | 1428 |
| mixed-150k-mod | 150000 | 0 | 424.015 | 529.670 | 721.455 | 840.928 | 2.4 | 1853 |
| mobs-25k-mod | 25000 | 0 | 262.162 | 459.315 | 578.143 | 634.137 | 3.8 | 1864 |
| spread-100k-mod | 100000 | 0 | 437.900 | 581.073 | 745.010 | 908.280 | 2.3 | 2104 |
| spread-100k | 100000 | 0 | 535.055 | 613.785 | 869.243 | 928.467 | 1.9 | 2182 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

