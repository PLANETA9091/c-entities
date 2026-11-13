# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 80.257 | 145.824 | 224.650 | 322.272 | 12.5 | 2044 |
| mobs-25k | 25000 | 0 | 574.886 | 643.196 | 720.094 | 759.256 | 1.7 | 1565 |
| mixed-150k | 150000 | 0 | 393.775 | 537.581 | 762.993 | 985.745 | 2.5 | 2575 |
| items-100k-mod | 100000 | 0 | 32.719 | 51.798 | 128.836 | 237.298 | 20.0 | 1500 |
| mixed-150k-mod | 150000 | 0 | 162.695 | 274.787 | 416.960 | 505.711 | 6.1 | 2153 |
| mobs-25k-mod | 25000 | 0 | 3511.266 | 3760.439 | 3928.370 | 4262.820 | 0.3 | 1976 |
| mobs-25k-native-mod | 25000 | 0 | 47.282 | 120.860 | 218.903 | 275.801 | 20.0 | 1424 |
| mobs-25k-native | 25000 | 0 | 507.017 | 594.385 | 678.304 | 770.224 | 2.0 | 1102 |
| spread-100k-mod | 100000 | 0 | 205.978 | 293.424 | 425.399 | 539.346 | 4.9 | 2284 |
| spread-100k | 100000 | 0 | 469.418 | 578.109 | 673.872 | 814.749 | 2.1 | 2011 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

