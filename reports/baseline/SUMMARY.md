# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.666 | 116.860 | 193.037 | 251.167 | 13.8 | 1366 |
| mobs-25k | 25000 | 0 | 493.331 | 552.959 | 649.609 | 770.142 | 2.0 | 993 |
| mixed-150k | 150000 | 0 | 523.693 | 722.189 | 928.367 | 1031.479 | 1.9 | 3097 |
| items-100k-mod | 100000 | 0 | 35.234 | 62.938 | 125.766 | 317.916 | 20.0 | 1674 |
| mixed-150k-mod | 150000 | 0 | 79.843 | 115.734 | 227.015 | 331.453 | 12.5 | 2201 |
| mobs-25k-mod | 25000 | 0 | 2449.264 | 2640.540 | 2803.364 | 2917.777 | 0.4 | 2154 |
| mobs-25k-native-mod | 25000 | 0 | 15.585 | 26.423 | 93.804 | 112.878 | 20.0 | 1528 |
| mobs-25k-native | 25000 | 0 | 290.957 | 354.501 | 446.683 | 549.377 | 3.4 | 1626 |
| spread-100k-mod | 100000 | 0 | 66.074 | 125.735 | 224.412 | 256.592 | 15.1 | 2263 |
| spread-100k | 100000 | 0 | 415.261 | 535.246 | 642.916 | 764.255 | 2.4 | 1845 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

