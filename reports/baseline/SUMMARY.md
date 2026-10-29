# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.471 | 83.162 | 152.382 | 210.840 | 15.3 | 1086 |
| mobs-25k | 25000 | 0 | 404.554 | 451.440 | 492.962 | 543.717 | 2.5 | 1757 |
| mixed-150k | 150000 | 0 | 525.669 | 632.385 | 844.942 | 955.500 | 1.9 | 2910 |
| items-100k-mod | 100000 | 0 | 32.181 | 87.669 | 162.324 | 330.043 | 20.0 | 1307 |
| mixed-150k-mod | 150000 | 0 | 317.432 | 413.025 | 530.595 | 550.423 | 3.2 | 1714 |
| mobs-25k-mod | 25000 | 0 | 160.363 | 309.406 | 560.547 | 752.139 | 6.2 | 1380 |
| spread-100k-mod | 100000 | 0 | 182.633 | 301.215 | 450.790 | 681.555 | 5.5 | 2768 |
| spread-100k | 100000 | 0 | 440.745 | 507.723 | 721.626 | 784.901 | 2.3 | 2231 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

