# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 59.448 | 81.857 | 137.508 | 189.868 | 16.8 | 1154 |
| mobs-25k | 25000 | 0 | 490.434 | 543.172 | 632.093 | 707.133 | 2.0 | 2150 |
| mixed-150k | 150000 | 0 | 657.535 | 790.589 | 1146.015 | 1239.791 | 1.5 | 1836 |
| items-100k-mod | 100000 | 0 | 22.835 | 59.286 | 130.101 | 178.264 | 20.0 | 1480 |
| mixed-150k-mod | 150000 | 0 | 414.159 | 523.539 | 679.178 | 778.236 | 2.4 | 1958 |
| mobs-25k-mod | 25000 | 0 | 304.925 | 545.334 | 663.048 | 751.116 | 3.3 | 1621 |
| spread-100k-mod | 100000 | 0 | 408.826 | 578.896 | 707.916 | 905.886 | 2.4 | 2546 |
| spread-100k | 100000 | 0 | 508.703 | 574.925 | 795.057 | 866.571 | 2.0 | 2166 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

