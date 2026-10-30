# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.083 | 85.168 | 154.152 | 195.842 | 15.1 | 1597 |
| mobs-25k | 25000 | 0 | 535.068 | 599.109 | 655.166 | 686.934 | 1.9 | 873 |
| mixed-150k | 150000 | 0 | 648.501 | 782.550 | 1158.388 | 1279.946 | 1.5 | 1921 |
| items-100k-mod | 100000 | 0 | 33.587 | 81.599 | 167.252 | 309.649 | 20.0 | 1122 |
| mixed-150k-mod | 150000 | 0 | 294.702 | 414.167 | 590.149 | 697.311 | 3.4 | 1602 |
| mobs-25k-mod | 25000 | 0 | 309.416 | 591.144 | 779.471 | 874.575 | 3.2 | 1647 |
| spread-100k-mod | 100000 | 0 | 286.334 | 446.470 | 664.743 | 820.268 | 3.5 | 1908 |
| spread-100k | 100000 | 0 | 501.602 | 581.885 | 795.948 | 884.908 | 2.0 | 2125 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

