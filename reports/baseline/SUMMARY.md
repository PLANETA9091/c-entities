# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.032 | 80.577 | 146.303 | 209.483 | 15.4 | 1513 |
| mobs-25k | 25000 | 0 | 525.143 | 628.721 | 725.906 | 773.256 | 1.9 | 922 |
| mixed-150k | 150000 | 0 | 549.782 | 693.749 | 1062.047 | 1198.439 | 1.8 | 1745 |
| items-100k-mod | 100000 | 0 | 13.073 | 17.846 | 83.977 | 107.964 | 20.0 | 1338 |
| mixed-150k-mod | 150000 | 0 | 308.784 | 409.615 | 570.461 | 663.077 | 3.2 | 1834 |
| mobs-25k-mod | 25000 | 0 | 261.360 | 454.521 | 695.094 | 724.281 | 3.8 | 1480 |
| spread-100k-mod | 100000 | 0 | 152.369 | 240.666 | 376.433 | 463.728 | 6.6 | 3038 |
| spread-100k | 100000 | 0 | 386.149 | 457.360 | 654.255 | 727.894 | 2.6 | 1782 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

