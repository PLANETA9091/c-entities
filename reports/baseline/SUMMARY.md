# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.588 | 88.364 | 159.372 | 197.039 | 14.2 | 2052 |
| mobs-25k | 25000 | 0 | 580.254 | 649.586 | 711.854 | 745.283 | 1.7 | 1089 |
| mixed-150k | 150000 | 0 | 624.193 | 746.783 | 944.434 | 1136.344 | 1.6 | 3024 |
| items-100k-mod | 100000 | 0 | 34.302 | 88.477 | 163.397 | 181.745 | 20.0 | 1417 |
| mixed-150k-mod | 150000 | 0 | 318.950 | 404.205 | 504.840 | 524.764 | 3.1 | 2135 |
| mobs-25k-mod | 25000 | 0 | 203.492 | 403.186 | 664.093 | 710.193 | 4.9 | 1386 |
| spread-100k-mod | 100000 | 0 | 174.395 | 329.658 | 523.550 | 679.047 | 5.7 | 2441 |
| spread-100k | 100000 | 0 | 616.001 | 691.884 | 950.472 | 1044.352 | 1.6 | 2240 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

