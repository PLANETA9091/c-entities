# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.337 | 87.067 | 160.556 | 221.281 | 14.4 | 879 |
| mobs-25k | 25000 | 0 | 320.414 | 365.193 | 426.120 | 473.700 | 3.1 | 1568 |
| mixed-150k | 150000 | 0 | 513.948 | 623.357 | 823.482 | 909.075 | 1.9 | 2803 |
| items-100k-mod | 100000 | 0 | 29.569 | 87.059 | 167.923 | 244.252 | 20.0 | 1550 |
| mixed-150k-mod | 150000 | 0 | 328.521 | 420.461 | 555.289 | 622.233 | 3.0 | 1828 |
| mobs-25k-mod | 25000 | 0 | 280.342 | 458.479 | 579.857 | 695.395 | 3.6 | 1656 |
| spread-100k-mod | 100000 | 0 | 434.548 | 539.813 | 684.401 | 826.074 | 2.3 | 2580 |
| spread-100k | 100000 | 0 | 527.518 | 593.615 | 842.425 | 903.008 | 1.9 | 2109 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

