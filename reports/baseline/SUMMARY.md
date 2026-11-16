# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.840 | 106.230 | 169.188 | 197.910 | 15.7 | 1112 |
| mobs-25k | 25000 | 0 | 342.271 | 418.391 | 524.028 | 704.927 | 2.9 | 1348 |
| mixed-150k | 150000 | 0 | 468.016 | 626.379 | 814.105 | 920.842 | 2.1 | 1770 |
| items-100k-mod | 100000 | 0 | 29.990 | 52.942 | 116.961 | 381.401 | 20.0 | 1785 |
| mixed-150k-mod | 150000 | 0 | 81.994 | 127.920 | 257.158 | 370.779 | 12.2 | 2343 |
| mobs-25k-mod | 25000 | 0 | 306.681 | 1525.957 | 2361.242 | 2784.365 | 3.3 | 1681 |
| mobs-25k-native-mod | 25000 | 0 | 19.097 | 35.506 | 112.364 | 151.773 | 20.0 | 1571 |
| mobs-25k-native | 25000 | 0 | 394.557 | 475.026 | 569.748 | 754.510 | 2.5 | 1493 |
| spread-100k-mod | 100000 | 0 | 50.026 | 73.415 | 185.551 | 254.539 | 20.0 | 2251 |
| spread-100k | 100000 | 0 | 303.789 | 376.979 | 472.526 | 583.855 | 3.3 | 2597 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

