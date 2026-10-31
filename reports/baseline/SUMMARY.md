# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.266 | 82.071 | 155.084 | 216.108 | 15.1 | 1484 |
| mobs-25k | 25000 | 0 | 511.936 | 575.095 | 627.207 | 742.906 | 2.0 | 1192 |
| mixed-150k | 150000 | 0 | 461.397 | 602.434 | 798.942 | 1080.232 | 2.2 | 2630 |
| items-100k-mod | 100000 | 0 | 33.623 | 92.141 | 157.768 | 182.480 | 20.0 | 1506 |
| mixed-150k-mod | 150000 | 0 | 231.970 | 337.363 | 466.072 | 562.539 | 4.3 | 2294 |
| mobs-25k-mod | 25000 | 0 | 274.305 | 436.573 | 584.133 | 675.052 | 3.6 | 1692 |
| spread-100k-mod | 100000 | 0 | 281.851 | 425.499 | 528.897 | 669.463 | 3.5 | 1855 |
| spread-100k | 100000 | 0 | 514.534 | 587.926 | 816.129 | 881.082 | 1.9 | 2272 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

