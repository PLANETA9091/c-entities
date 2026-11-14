# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 87.565 | 150.017 | 230.301 | 308.848 | 11.4 | 1718 |
| mobs-25k | 25000 | 0 | 441.846 | 502.449 | 636.130 | 744.179 | 2.3 | 1871 |
| mixed-150k | 150000 | 0 | 766.340 | 1114.817 | 1390.453 | 1622.570 | 1.3 | 2683 |
| items-100k-mod | 100000 | 0 | 67.541 | 114.263 | 248.803 | 329.596 | 14.8 | 2234 |
| mixed-150k-mod | 150000 | 0 | 111.008 | 182.486 | 349.019 | 515.771 | 9.0 | 2731 |
| mobs-25k-mod | 25000 | 0 | 2413.385 | 2594.400 | 2711.350 | 2815.168 | 0.4 | 1666 |
| mobs-25k-native-mod | 25000 | 0 | 20.590 | 37.765 | 109.339 | 214.452 | 20.0 | 1595 |
| mobs-25k-native | 25000 | 0 | 493.739 | 576.403 | 646.292 | 698.076 | 2.0 | 1877 |
| spread-100k-mod | 100000 | 0 | 89.980 | 138.245 | 271.531 | 379.883 | 11.1 | 2213 |
| spread-100k | 100000 | 0 | 527.492 | 616.161 | 695.559 | 756.825 | 1.9 | 2364 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

