# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.427 | 82.071 | 149.839 | 189.073 | 14.8 | 1352 |
| mobs-25k | 25000 | 0 | 504.356 | 577.065 | 631.751 | 680.990 | 2.0 | 1353 |
| mixed-150k | 150000 | 0 | 673.345 | 798.130 | 1062.096 | 1236.895 | 1.5 | 2115 |
| items-100k-mod | 100000 | 0 | 29.127 | 74.785 | 154.514 | 214.606 | 20.0 | 1590 |
| mixed-150k-mod | 150000 | 0 | 287.995 | 399.814 | 538.782 | 626.314 | 3.5 | 1661 |
| mobs-25k-mod | 25000 | 0 | 269.546 | 556.349 | 673.904 | 739.413 | 3.7 | 1667 |
| spread-100k-mod | 100000 | 0 | 299.264 | 470.850 | 591.360 | 805.765 | 3.3 | 1935 |
| spread-100k | 100000 | 0 | 539.155 | 600.388 | 847.135 | 920.320 | 1.9 | 2120 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

