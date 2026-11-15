# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.254 | 126.424 | 181.602 | 256.519 | 14.4 | 1272 |
| mobs-25k | 25000 | 0 | 467.585 | 546.043 | 639.198 | 702.284 | 2.1 | 1258 |
| mixed-150k | 150000 | 0 | 474.696 | 647.640 | 830.536 | 1002.128 | 2.1 | 3130 |
| items-100k-mod | 100000 | 0 | 31.766 | 52.823 | 143.918 | 218.490 | 20.0 | 1714 |
| mixed-150k-mod | 150000 | 0 | 87.781 | 167.368 | 322.681 | 413.356 | 11.4 | 2156 |
| mobs-25k-mod | 25000 | 0 | 1988.677 | 2113.616 | 2228.011 | 2305.855 | 0.5 | 1846 |
| mobs-25k-native-mod | 25000 | 0 | 25.242 | 51.915 | 123.484 | 249.448 | 20.0 | 1689 |
| mobs-25k-native | 25000 | 0 | 531.839 | 602.080 | 704.406 | 813.773 | 1.9 | 1167 |
| spread-100k-mod | 100000 | 0 | 61.116 | 109.410 | 206.715 | 288.843 | 16.4 | 2525 |
| spread-100k | 100000 | 0 | 508.494 | 597.480 | 703.732 | 799.452 | 2.0 | 2340 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

