# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| control | 0 | 0 | 0.101 | 0.139 | 0.264 | 4.815 | 20.0 | 552 |
| items-25k | 25000 | 0 | 20.956 | 22.812 | 35.017 | 58.650 | 20.0 | 939 |
| items-100k | 100000 | 0 | 59.686 | 68.519 | 102.132 | 158.616 | 16.8 | 2217 |
| mobs-10k | 10000 | 0 | 112.816 | 120.821 | 152.061 | 158.883 | 8.9 | 1108 |
| mobs-25k | 25000 | 0 | 505.493 | 540.977 | 580.703 | 605.899 | 2.0 | 1152 |
| mixed-50k | 50000 | 0 | 119.158 | 130.815 | 156.934 | 172.308 | 8.4 | 919 |
| mixed-150k | 150000 | 0 | 505.837 | 577.145 | 648.826 | 699.294 | 2.0 | 1587 |
| tiles-25k | 0 | 24964 | 10.032 | 10.312 | 16.784 | 25.405 | 20.0 | 531 |
| tiles-100k | 0 | 68121 | 22.567 | 26.562 | 39.296 | 51.486 | 20.0 | 1095 |
| items-100k-mod | 100000 | 0 | 69.748 | 76.625 | 115.499 | 181.553 | 14.3 | 2250 |
| mixed-150k-mod | 150000 | 0 | 864.640 | 970.629 | 1108.337 | 1205.123 | 1.2 | 1722 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

