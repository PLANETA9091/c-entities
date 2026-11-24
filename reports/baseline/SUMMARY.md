# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.231 | 121.563 | 179.753 | 230.923 | 14.2 | 1844 |
| mobs-25k | 25000 | 0 | 590.426 | 692.891 | 831.574 | 1056.112 | 1.7 | 1634 |
| mixed-150k | 150000 | 0 | 552.133 | 753.650 | 1012.769 | 1138.337 | 1.8 | 1530 |
| items-100k-mod | 100000 | 0 | 14.509 | 26.499 | 98.930 | 254.108 | 20.0 | 1377 |
| mixed-150k-mod | 150000 | 0 | 56.027 | 99.069 | 226.701 | 327.543 | 17.8 | 1720 |
| mobs-25k-mod | 25000 | 0 | 3191.324 | 3511.751 | 3892.618 | 4020.240 | 0.3 | 1676 |
| mobs-25k-native-mod | 25000 | 0 | 18.765 | 36.222 | 107.397 | 139.789 | 20.0 | 1398 |
| mobs-25k-native | 25000 | 0 | 317.426 | 396.191 | 450.502 | 485.914 | 3.2 | 1148 |
| spread-100k-mod | 100000 | 0 | 39.088 | 60.730 | 187.227 | 249.423 | 20.0 | 2122 |
| spread-100k | 100000 | 0 | 389.223 | 468.870 | 564.927 | 668.246 | 2.6 | 2192 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

