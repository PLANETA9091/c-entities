# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 101.185 | 119.616 | 155.864 | 234.955 | 9.9 | 1164 |
| mobs-25k | 25000 | 0 | 575.255 | 627.898 | 682.374 | 727.107 | 1.7 | 1132 |
| mixed-150k | 150000 | 0 | 773.454 | 847.409 | 916.702 | 978.240 | 1.3 | 1754 |
| items-100k-mod | 100000 | 0 | 63.039 | 66.826 | 87.954 | 137.601 | 15.9 | 1530 |
| mixed-150k-mod | 150000 | 0 | 646.418 | 703.514 | 750.751 | 791.172 | 1.5 | 2542 |
| mobs-25k-mod | 25000 | 0 | 350.550 | 416.555 | 467.500 | 524.149 | 2.9 | 1616 |
| spread-100k-mod | 100000 | 0 | 307.950 | 380.476 | 451.232 | 534.740 | 3.2 | 2169 |
| spread-100k | 100000 | 0 | 403.884 | 465.305 | 543.383 | 590.082 | 2.5 | 2294 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

