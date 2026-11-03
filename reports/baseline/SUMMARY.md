# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 76.746 | 97.714 | 173.803 | 205.569 | 13.0 | 1215 |
| mobs-25k | 25000 | 0 | 521.028 | 575.092 | 634.003 | 652.791 | 1.9 | 930 |
| mixed-150k | 150000 | 0 | 631.328 | 757.240 | 1048.903 | 1219.411 | 1.6 | 1630 |
| items-100k-mod | 100000 | 0 | 28.954 | 69.272 | 150.263 | 329.075 | 20.0 | 1467 |
| mixed-150k-mod | 150000 | 0 | 413.742 | 519.495 | 684.663 | 759.528 | 2.4 | 1926 |
| mobs-25k-mod | 25000 | 0 | 266.834 | 423.158 | 564.543 | 605.912 | 3.7 | 1329 |
| spread-100k-mod | 100000 | 0 | 593.524 | 730.708 | 957.904 | 1160.651 | 1.7 | 2094 |
| spread-100k | 100000 | 0 | 713.666 | 806.400 | 1113.673 | 1196.497 | 1.4 | 2095 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

