# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 56.506 | 71.800 | 128.866 | 229.998 | 17.7 | 1588 |
| mobs-25k | 25000 | 0 | 518.540 | 574.728 | 624.272 | 660.576 | 1.9 | 1351 |
| mixed-150k | 150000 | 0 | 511.780 | 626.552 | 968.828 | 1131.174 | 2.0 | 2914 |
| items-100k-mod | 100000 | 0 | 20.798 | 51.512 | 111.322 | 178.396 | 20.0 | 1180 |
| mixed-150k-mod | 150000 | 0 | 221.484 | 335.668 | 478.360 | 604.038 | 4.5 | 2413 |
| mobs-25k-mod | 25000 | 0 | 294.775 | 586.139 | 742.026 | 896.222 | 3.4 | 1687 |
| spread-100k-mod | 100000 | 0 | 295.773 | 453.509 | 547.410 | 743.248 | 3.4 | 2365 |
| spread-100k | 100000 | 0 | 540.946 | 617.861 | 877.626 | 971.713 | 1.8 | 2105 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

