# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.010 | 112.576 | 165.475 | 251.279 | 14.9 | 1634 |
| mobs-25k | 25000 | 0 | 376.536 | 447.827 | 545.066 | 675.699 | 2.7 | 1501 |
| mixed-150k | 150000 | 0 | 464.850 | 640.755 | 853.667 | 975.955 | 2.2 | 2903 |
| items-100k-mod | 100000 | 0 | 34.603 | 62.130 | 125.554 | 201.948 | 20.0 | 1609 |
| mixed-150k-mod | 150000 | 0 | 90.555 | 138.749 | 337.411 | 423.020 | 11.0 | 2232 |
| mobs-25k-mod | 25000 | 0 | 2026.367 | 2150.043 | 2292.662 | 2390.557 | 0.5 | 1700 |
| mobs-25k-native-mod | 25000 | 0 | 19.018 | 34.531 | 101.506 | 196.927 | 20.0 | 1645 |
| mobs-25k-native | 25000 | 0 | 295.242 | 354.981 | 440.211 | 505.535 | 3.4 | 1426 |
| spread-100k-mod | 100000 | 0 | 62.112 | 104.354 | 219.498 | 334.573 | 16.1 | 2342 |
| spread-100k | 100000 | 0 | 512.366 | 613.378 | 689.255 | 794.236 | 2.0 | 2022 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

