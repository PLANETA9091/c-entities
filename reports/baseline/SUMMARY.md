# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 86.649 | 132.151 | 217.071 | 286.604 | 11.5 | 1907 |
| mobs-25k | 25000 | 0 | 235.115 | 266.209 | 306.138 | 328.230 | 4.3 | 1199 |
| mixed-150k | 150000 | 0 | 395.284 | 473.242 | 636.170 | 738.461 | 2.5 | 2232 |
| items-100k-mod | 100000 | 0 | 16.106 | 20.987 | 98.692 | 150.551 | 20.0 | 1139 |
| mixed-150k-mod | 150000 | 0 | 233.602 | 283.318 | 340.712 | 411.871 | 4.3 | 2925 |
| mobs-25k-mod | 25000 | 0 | 137.660 | 199.520 | 249.496 | 295.526 | 7.3 | 1570 |
| spread-100k-mod | 100000 | 0 | 163.731 | 246.015 | 318.212 | 419.082 | 6.1 | 2493 |
| spread-100k | 100000 | 0 | 508.615 | 581.774 | 874.724 | 989.525 | 2.0 | 1980 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

