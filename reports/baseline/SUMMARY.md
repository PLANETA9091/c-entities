# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.270 | 116.963 | 176.332 | 304.920 | 14.0 | 2047 |
| mobs-25k | 25000 | 0 | 546.106 | 621.493 | 704.883 | 966.814 | 1.8 | 1194 |
| mixed-150k | 150000 | 0 | 637.944 | 861.258 | 1116.280 | 1235.337 | 1.6 | 2129 |
| items-100k-mod | 100000 | 0 | 28.628 | 51.975 | 115.001 | 189.564 | 20.0 | 1650 |
| mixed-150k-mod | 150000 | 0 | 96.253 | 160.584 | 315.375 | 401.030 | 10.4 | 2120 |
| mobs-25k-mod | 25000 | 0 | 392.595 | 1969.514 | 3034.671 | 3368.145 | 2.5 | 1621 |
| mobs-25k-native-mod | 25000 | 0 | 19.409 | 34.198 | 108.906 | 130.581 | 20.0 | 1670 |
| mobs-25k-native | 25000 | 0 | 391.617 | 460.551 | 573.640 | 683.521 | 2.6 | 938 |
| spread-100k-mod | 100000 | 0 | 57.283 | 86.365 | 207.798 | 435.496 | 17.5 | 2134 |
| spread-100k | 100000 | 0 | 373.237 | 457.095 | 557.954 | 632.396 | 2.7 | 1777 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

