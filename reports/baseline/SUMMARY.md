# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.519 | 85.015 | 155.281 | 208.822 | 14.6 | 1575 |
| mobs-25k | 25000 | 0 | 508.423 | 573.909 | 626.894 | 645.262 | 2.0 | 1904 |
| mixed-150k | 150000 | 0 | 670.306 | 821.119 | 1226.387 | 1383.389 | 1.5 | 2517 |
| items-100k-mod | 100000 | 0 | 27.971 | 74.329 | 163.197 | 250.990 | 20.0 | 1415 |
| mixed-150k-mod | 150000 | 0 | 281.342 | 406.124 | 593.003 | 697.292 | 3.6 | 1961 |
| mobs-25k-mod | 25000 | 0 | 282.837 | 471.536 | 619.832 | 716.156 | 3.5 | 1860 |
| spread-100k-mod | 100000 | 0 | 304.335 | 466.025 | 633.793 | 818.520 | 3.3 | 2160 |
| spread-100k | 100000 | 0 | 552.374 | 625.484 | 893.177 | 949.122 | 1.8 | 2030 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

