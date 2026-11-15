# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.994 | 115.477 | 176.985 | 467.302 | 14.3 | 2184 |
| mobs-25k | 25000 | 0 | 493.659 | 560.098 | 650.751 | 786.368 | 2.0 | 1251 |
| mixed-150k | 150000 | 0 | 631.243 | 844.539 | 1074.289 | 1237.865 | 1.6 | 2032 |
| items-100k-mod | 100000 | 0 | 34.544 | 55.879 | 160.970 | 342.987 | 20.0 | 1911 |
| mixed-150k-mod | 150000 | 0 | 103.070 | 179.553 | 341.541 | 429.627 | 9.7 | 2126 |
| mobs-25k-mod | 25000 | 0 | 2422.432 | 2610.300 | 2769.104 | 2925.929 | 0.4 | 1886 |
| mobs-25k-native-mod | 25000 | 0 | 21.498 | 39.032 | 114.032 | 155.151 | 20.0 | 1741 |
| mobs-25k-native | 25000 | 0 | 507.605 | 586.417 | 719.862 | 785.171 | 2.0 | 1765 |
| spread-100k-mod | 100000 | 0 | 61.017 | 113.787 | 205.680 | 230.918 | 16.4 | 2390 |
| spread-100k | 100000 | 0 | 384.763 | 472.219 | 573.967 | 674.942 | 2.6 | 2406 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

