# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 74.910 | 120.196 | 187.330 | 251.786 | 13.3 | 1837 |
| mobs-25k | 25000 | 0 | 301.805 | 382.406 | 476.424 | 652.739 | 3.3 | 1112 |
| mixed-150k | 150000 | 0 | 441.507 | 626.158 | 874.467 | 1041.033 | 2.3 | 2419 |
| items-100k-mod | 100000 | 0 | 30.824 | 52.827 | 143.906 | 346.177 | 20.0 | 1770 |
| mixed-150k-mod | 150000 | 0 | 173.946 | 311.920 | 463.489 | 692.499 | 5.7 | 2252 |
| mobs-25k-mod | 25000 | 0 | 1859.186 | 1968.082 | 2079.309 | 2148.256 | 0.5 | 1902 |
| mobs-25k-native-mod | 25000 | 0 | 23.429 | 43.027 | 117.336 | 143.973 | 20.0 | 1492 |
| mobs-25k-native | 25000 | 0 | 396.851 | 467.118 | 541.133 | 714.761 | 2.5 | 1236 |
| spread-100k-mod | 100000 | 0 | 159.343 | 234.414 | 373.451 | 517.749 | 6.3 | 2338 |
| spread-100k | 100000 | 0 | 362.727 | 451.981 | 546.132 | 609.082 | 2.8 | 1728 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

