# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 78.396 | 100.291 | 189.675 | 241.718 | 12.8 | 1767 |
| mobs-25k | 25000 | 0 | 396.515 | 438.509 | 477.371 | 511.117 | 2.5 | 1931 |
| mixed-150k | 150000 | 0 | 711.735 | 895.291 | 1263.973 | 1566.794 | 1.4 | 3283 |
| items-100k-mod | 100000 | 0 | 27.014 | 88.862 | 166.492 | 270.849 | 20.0 | 1241 |
| mixed-150k-mod | 150000 | 0 | 402.540 | 500.997 | 767.700 | 858.183 | 2.5 | 1744 |
| mobs-25k-mod | 25000 | 0 | 156.106 | 295.035 | 543.399 | 593.492 | 6.4 | 1918 |
| spread-100k-mod | 100000 | 0 | 161.212 | 269.312 | 384.405 | 454.927 | 6.2 | 2605 |
| spread-100k | 100000 | 0 | 372.288 | 436.863 | 605.955 | 671.714 | 2.7 | 1827 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

