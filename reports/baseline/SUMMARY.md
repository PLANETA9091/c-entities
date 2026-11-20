# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.956 | 102.126 | 158.220 | 232.699 | 15.2 | 801 |
| mobs-25k | 25000 | 0 | 330.285 | 407.527 | 506.877 | 617.544 | 3.0 | 1571 |
| mixed-150k | 150000 | 0 | 549.588 | 763.109 | 1011.986 | 1133.748 | 1.8 | 1720 |
| items-100k-mod | 100000 | 0 | 13.125 | 22.276 | 83.813 | 192.788 | 20.0 | 1354 |
| mixed-150k-mod | 150000 | 0 | 67.412 | 113.776 | 286.798 | 394.271 | 14.8 | 1735 |
| mobs-25k-mod | 25000 | 0 | 1742.799 | 2120.820 | 2324.752 | 2924.424 | 0.6 | 1973 |
| mobs-25k-native-mod | 25000 | 0 | 17.763 | 27.851 | 103.408 | 143.749 | 20.0 | 1180 |
| mobs-25k-native | 25000 | 0 | 511.416 | 591.491 | 674.429 | 746.534 | 2.0 | 1377 |
| spread-100k-mod | 100000 | 0 | 54.239 | 93.797 | 191.777 | 264.382 | 18.4 | 1955 |
| spread-100k | 100000 | 0 | 505.448 | 600.584 | 686.470 | 861.288 | 2.0 | 2145 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

