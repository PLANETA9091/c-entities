# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.390 | 90.153 | 151.767 | 191.654 | 15.3 | 1390 |
| mobs-25k | 25000 | 0 | 717.134 | 790.934 | 843.467 | 895.023 | 1.4 | 950 |
| mixed-150k | 150000 | 0 | 684.953 | 819.706 | 1218.993 | 1340.688 | 1.5 | 1924 |
| items-100k-mod | 100000 | 0 | 23.084 | 52.035 | 119.162 | 194.440 | 20.0 | 1417 |
| mixed-150k-mod | 150000 | 0 | 319.326 | 427.366 | 629.008 | 747.155 | 3.1 | 1913 |
| mobs-25k-mod | 25000 | 0 | 264.332 | 448.667 | 576.363 | 658.208 | 3.8 | 1683 |
| spread-100k-mod | 100000 | 0 | 296.908 | 484.879 | 601.623 | 849.167 | 3.4 | 2171 |
| spread-100k | 100000 | 0 | 557.940 | 643.332 | 918.896 | 975.433 | 1.8 | 2153 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

