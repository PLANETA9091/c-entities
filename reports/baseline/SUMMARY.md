# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.458 | 84.586 | 153.006 | 189.428 | 14.4 | 1196 |
| mobs-25k | 25000 | 0 | 505.831 | 555.000 | 601.468 | 670.104 | 2.0 | 850 |
| mixed-150k | 150000 | 0 | 640.585 | 811.644 | 1137.187 | 1302.032 | 1.6 | 3159 |
| items-100k-mod | 100000 | 0 | 33.662 | 75.458 | 138.117 | 190.663 | 20.0 | 1289 |
| mixed-150k-mod | 150000 | 0 | 407.962 | 501.696 | 692.876 | 838.171 | 2.5 | 1969 |
| mobs-25k-mod | 25000 | 0 | 319.379 | 567.651 | 706.020 | 766.607 | 3.1 | 1579 |
| spread-100k-mod | 100000 | 0 | 385.981 | 530.482 | 659.466 | 867.216 | 2.6 | 2649 |
| spread-100k | 100000 | 0 | 472.365 | 540.086 | 737.360 | 838.233 | 2.1 | 2110 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

