# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 61.532 | 72.873 | 136.137 | 180.727 | 16.3 | 1520 |
| mobs-25k | 25000 | 0 | 390.208 | 445.582 | 494.833 | 512.371 | 2.6 | 1063 |
| mixed-150k | 150000 | 0 | 361.040 | 435.373 | 581.182 | 712.833 | 2.8 | 2430 |
| items-100k-mod | 100000 | 0 | 8.975 | 12.571 | 41.475 | 103.109 | 20.0 | 1286 |
| mixed-150k-mod | 150000 | 0 | 227.173 | 263.545 | 313.443 | 364.110 | 4.4 | 2027 |
| mobs-25k-mod | 25000 | 0 | 205.468 | 301.349 | 392.735 | 447.715 | 4.9 | 1709 |
| spread-100k-mod | 100000 | 0 | 145.065 | 217.957 | 310.275 | 379.387 | 6.9 | 2222 |
| spread-100k | 100000 | 0 | 413.473 | 475.768 | 695.385 | 729.052 | 2.4 | 2079 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

