# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 58.800 | 71.605 | 128.684 | 178.230 | 17.0 | 1110 |
| mobs-25k | 25000 | 0 | 376.947 | 419.030 | 456.138 | 471.071 | 2.7 | 1060 |
| mixed-150k | 150000 | 0 | 507.215 | 644.862 | 952.128 | 1061.673 | 2.0 | 2914 |
| items-100k-mod | 100000 | 0 | 14.389 | 18.451 | 71.920 | 105.321 | 20.0 | 1213 |
| mixed-150k-mod | 150000 | 0 | 297.919 | 347.019 | 405.712 | 466.403 | 3.4 | 2436 |
| mobs-25k-mod | 25000 | 0 | 185.987 | 292.340 | 369.181 | 419.746 | 5.4 | 1365 |
| spread-100k-mod | 100000 | 0 | 174.291 | 250.143 | 321.537 | 433.371 | 5.7 | 1893 |
| spread-100k | 100000 | 0 | 428.279 | 494.549 | 723.062 | 787.706 | 2.3 | 2090 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

