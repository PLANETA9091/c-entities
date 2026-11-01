# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 61.496 | 79.044 | 140.186 | 208.861 | 16.3 | 1201 |
| mobs-25k | 25000 | 0 | 479.530 | 536.926 | 593.580 | 658.627 | 2.1 | 1101 |
| mixed-150k | 150000 | 0 | 529.003 | 649.527 | 852.756 | 1007.974 | 1.9 | 2730 |
| items-100k-mod | 100000 | 0 | 25.723 | 66.086 | 141.604 | 192.914 | 20.0 | 1232 |
| mixed-150k-mod | 150000 | 0 | 235.272 | 332.622 | 471.582 | 579.215 | 4.3 | 1781 |
| mobs-25k-mod | 25000 | 0 | 254.348 | 499.489 | 643.756 | 812.611 | 3.9 | 1384 |
| spread-100k-mod | 100000 | 0 | 275.376 | 415.709 | 587.239 | 737.539 | 3.6 | 2081 |
| spread-100k | 100000 | 0 | 490.981 | 557.479 | 777.430 | 845.877 | 2.0 | 2158 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

