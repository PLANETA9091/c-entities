# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.152 | 116.852 | 173.338 | 201.010 | 14.5 | 1369 |
| mobs-25k | 25000 | 0 | 475.204 | 554.497 | 644.416 | 713.995 | 2.1 | 1295 |
| mixed-150k | 150000 | 0 | 472.037 | 632.042 | 800.971 | 901.137 | 2.1 | 3020 |
| items-100k-mod | 100000 | 0 | 29.698 | 49.211 | 116.253 | 212.515 | 20.0 | 1717 |
| mixed-150k-mod | 150000 | 0 | 88.514 | 150.587 | 306.685 | 388.239 | 11.3 | 2210 |
| mobs-25k-mod | 25000 | 0 | 1993.838 | 2105.783 | 2237.701 | 2265.882 | 0.5 | 2118 |
| mobs-25k-native-mod | 25000 | 0 | 20.312 | 36.164 | 107.067 | 148.135 | 20.0 | 1720 |
| mobs-25k-native | 25000 | 0 | 511.886 | 592.828 | 711.169 | 781.395 | 2.0 | 1410 |
| spread-100k-mod | 100000 | 0 | 63.700 | 125.412 | 230.341 | 314.799 | 15.7 | 2480 |
| spread-100k | 100000 | 0 | 525.006 | 631.295 | 725.034 | 851.907 | 1.9 | 2448 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

