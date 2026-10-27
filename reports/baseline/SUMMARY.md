# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.077 | 90.348 | 158.821 | 213.439 | 14.7 | 2177 |
| mobs-25k | 25000 | 0 | 387.933 | 436.189 | 488.194 | 536.865 | 2.6 | 1071 |
| mixed-150k | 150000 | 0 | 422.610 | 508.462 | 708.682 | 777.224 | 2.4 | 2414 |
| items-100k-mod | 100000 | 0 | 13.541 | 18.696 | 86.652 | 127.727 | 20.0 | 1154 |
| mixed-150k-mod | 150000 | 0 | 245.299 | 327.937 | 443.122 | 517.811 | 4.1 | 2954 |
| mobs-25k-mod | 25000 | 0 | 198.765 | 355.292 | 574.137 | 622.378 | 5.0 | 1451 |
| spread-100k-mod | 100000 | 0 | 143.498 | 239.521 | 370.768 | 451.970 | 7.0 | 2240 |
| spread-100k | 100000 | 0 | 335.655 | 396.018 | 584.264 | 635.048 | 3.0 | 2730 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

