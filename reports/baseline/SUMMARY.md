# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.995 | 89.390 | 169.180 | 244.615 | 13.7 | 1297 |
| mobs-25k | 25000 | 0 | 485.654 | 536.092 | 584.492 | 614.898 | 2.1 | 862 |
| mixed-150k | 150000 | 0 | 890.681 | 1081.086 | 1443.747 | 1573.261 | 1.1 | 2950 |
| items-100k-mod | 100000 | 0 | 32.135 | 91.951 | 169.987 | 242.076 | 20.0 | 1381 |
| mixed-150k-mod | 150000 | 0 | 232.451 | 337.471 | 506.172 | 558.436 | 4.3 | 1926 |
| mobs-25k-mod | 25000 | 0 | 271.981 | 485.805 | 612.476 | 663.008 | 3.7 | 2000 |
| spread-100k-mod | 100000 | 0 | 209.891 | 334.537 | 422.519 | 542.426 | 4.8 | 2097 |
| spread-100k | 100000 | 0 | 420.021 | 472.007 | 680.909 | 710.643 | 2.4 | 2410 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

