# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.203 | 130.717 | 197.491 | 244.066 | 13.8 | 2199 |
| mobs-25k | 25000 | 0 | 400.119 | 479.099 | 581.916 | 784.673 | 2.5 | 1173 |
| mixed-150k | 150000 | 0 | 791.221 | 1065.310 | 1468.651 | 1732.176 | 1.3 | 3102 |
| items-100k-mod | 100000 | 0 | 28.456 | 45.386 | 111.418 | 188.418 | 20.0 | 1685 |
| mixed-150k-mod | 150000 | 0 | 184.730 | 338.336 | 544.967 | 829.120 | 5.4 | 2123 |
| mobs-25k-mod | 25000 | 0 | 229.526 | 428.543 | 694.707 | 844.686 | 4.4 | 2278 |
| mobs-25k-native-mod | 25000 | 0 | 38.080 | 61.902 | 201.043 | 221.962 | 20.0 | 1486 |
| mobs-25k-native | 25000 | 0 | 542.652 | 626.206 | 726.081 | 818.369 | 1.8 | 1675 |
| spread-100k-mod | 100000 | 0 | 180.167 | 254.211 | 387.157 | 513.723 | 5.6 | 2490 |
| spread-100k | 100000 | 0 | 474.250 | 570.636 | 678.607 | 791.820 | 2.1 | 2257 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

