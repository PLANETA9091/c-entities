# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 84.202 | 156.117 | 231.761 | 314.772 | 11.9 | 1183 |
| mobs-25k | 25000 | 0 | 580.851 | 659.867 | 739.170 | 888.639 | 1.7 | 1134 |
| mixed-150k | 150000 | 0 | 569.137 | 767.541 | 977.394 | 1111.455 | 1.8 | 1878 |
| items-100k-mod | 100000 | 0 | 29.856 | 45.364 | 124.367 | 241.785 | 20.0 | 1452 |
| mixed-150k-mod | 150000 | 0 | 155.419 | 255.172 | 373.917 | 434.106 | 6.4 | 3064 |
| mobs-25k-mod | 25000 | 0 | 3534.786 | 3804.843 | 4088.265 | 4309.439 | 0.3 | 2001 |
| mobs-25k-native-mod | 25000 | 0 | 45.469 | 77.241 | 223.696 | 252.263 | 20.0 | 1632 |
| mobs-25k-native | 25000 | 0 | 457.736 | 522.509 | 632.297 | 815.442 | 2.2 | 2029 |
| spread-100k-mod | 100000 | 0 | 163.295 | 217.876 | 343.291 | 394.437 | 6.1 | 2049 |
| spread-100k | 100000 | 0 | 514.533 | 629.772 | 732.780 | 889.462 | 1.9 | 1900 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

