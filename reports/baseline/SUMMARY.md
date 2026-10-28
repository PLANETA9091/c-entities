# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.864 | 87.072 | 156.846 | 220.361 | 14.3 | 1552 |
| mobs-25k | 25000 | 0 | 531.780 | 590.544 | 650.524 | 702.845 | 1.9 | 1170 |
| mixed-150k | 150000 | 0 | 458.682 | 578.000 | 766.833 | 979.351 | 2.2 | 2730 |
| items-100k-mod | 100000 | 0 | 35.163 | 94.164 | 169.089 | 229.015 | 20.0 | 1260 |
| mixed-150k-mod | 150000 | 0 | 281.879 | 382.902 | 487.285 | 702.062 | 3.5 | 2011 |
| mobs-25k-mod | 25000 | 0 | 203.982 | 375.066 | 612.757 | 702.810 | 4.9 | 1454 |
| spread-100k-mod | 100000 | 0 | 198.898 | 346.521 | 563.882 | 707.617 | 5.0 | 1890 |
| spread-100k | 100000 | 0 | 508.713 | 581.538 | 812.001 | 905.495 | 2.0 | 2153 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

