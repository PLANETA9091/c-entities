# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.016 | 81.089 | 150.986 | 191.297 | 15.6 | 970 |
| mobs-25k | 25000 | 0 | 512.813 | 576.832 | 627.212 | 692.211 | 2.0 | 2140 |
| mixed-150k | 150000 | 0 | 457.190 | 552.905 | 782.086 | 866.538 | 2.2 | 2818 |
| items-100k-mod | 100000 | 0 | 32.131 | 90.310 | 138.300 | 251.159 | 20.0 | 1375 |
| mixed-150k-mod | 150000 | 0 | 344.909 | 451.590 | 718.967 | 807.666 | 2.9 | 1964 |
| mobs-25k-mod | 25000 | 0 | 212.311 | 465.359 | 762.457 | 822.927 | 4.7 | 2726 |
| spread-100k-mod | 100000 | 0 | 186.392 | 339.230 | 512.739 | 675.454 | 5.4 | 1914 |
| spread-100k | 100000 | 0 | 627.548 | 691.212 | 965.639 | 1038.555 | 1.6 | 2000 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

