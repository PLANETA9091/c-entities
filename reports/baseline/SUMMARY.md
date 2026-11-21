# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.096 | 99.839 | 175.093 | 214.312 | 15.6 | 1685 |
| mobs-25k | 25000 | 0 | 392.109 | 465.276 | 576.837 | 833.480 | 2.6 | 1349 |
| mixed-150k | 150000 | 0 | 456.187 | 631.798 | 847.412 | 926.261 | 2.2 | 2850 |
| items-100k-mod | 100000 | 0 | 11.593 | 21.282 | 92.369 | 198.860 | 20.0 | 1333 |
| mixed-150k-mod | 150000 | 0 | 58.923 | 103.753 | 233.214 | 398.216 | 17.0 | 1950 |
| mobs-25k-mod | 25000 | 0 | 2307.122 | 2619.845 | 2804.639 | 2993.832 | 0.4 | 1932 |
| mobs-25k-native-mod | 25000 | 0 | 19.826 | 34.499 | 112.963 | 193.262 | 20.0 | 1394 |
| mobs-25k-native | 25000 | 0 | 523.724 | 600.139 | 688.158 | 752.747 | 1.9 | 1805 |
| spread-100k-mod | 100000 | 0 | 49.272 | 99.456 | 213.182 | 299.241 | 20.0 | 2103 |
| spread-100k | 100000 | 0 | 517.528 | 612.309 | 711.945 | 910.047 | 1.9 | 2335 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

