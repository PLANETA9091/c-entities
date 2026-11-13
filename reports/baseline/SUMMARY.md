# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.057 | 121.469 | 174.427 | 256.362 | 14.1 | 1819 |
| mobs-25k | 25000 | 0 | 526.861 | 607.497 | 702.903 | 868.875 | 1.9 | 1223 |
| mixed-150k | 150000 | 0 | 479.827 | 652.376 | 894.767 | 1005.427 | 2.1 | 3100 |
| items-100k-mod | 100000 | 0 | 29.355 | 53.696 | 124.949 | 276.307 | 20.0 | 1743 |
| mixed-150k-mod | 150000 | 0 | 180.626 | 333.692 | 503.609 | 742.681 | 5.5 | 2561 |
| mobs-25k-mod | 25000 | 0 | 2047.420 | 2178.836 | 2364.899 | 2467.607 | 0.5 | 1985 |
| mobs-25k-native-mod | 25000 | 0 | 43.490 | 75.114 | 226.346 | 371.495 | 20.0 | 1455 |
| mobs-25k-native | 25000 | 0 | 505.159 | 573.758 | 671.909 | 920.272 | 2.0 | 1369 |
| spread-100k-mod | 100000 | 0 | 173.986 | 269.383 | 410.160 | 447.785 | 5.7 | 2904 |
| spread-100k | 100000 | 0 | 521.950 | 613.192 | 714.071 | 761.015 | 1.9 | 1820 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

