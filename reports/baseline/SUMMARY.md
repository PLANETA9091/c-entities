# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 60.775 | 78.300 | 145.824 | 170.901 | 16.5 | 2347 |
| mobs-25k | 25000 | 0 | 375.416 | 423.317 | 474.606 | 507.521 | 2.7 | 1230 |
| mixed-150k | 150000 | 0 | 378.171 | 463.758 | 649.641 | 745.368 | 2.6 | 2573 |
| items-100k-mod | 100000 | 0 | 34.309 | 56.732 | 145.792 | 188.078 | 20.0 | 1310 |
| mixed-150k-mod | 150000 | 0 | 341.605 | 397.292 | 450.030 | 513.076 | 2.9 | 2365 |
| mobs-25k-mod | 25000 | 0 | 196.741 | 238.505 | 275.311 | 302.296 | 5.1 | 1080 |
| spread-100k-mod | 100000 | 0 | 272.140 | 330.648 | 463.039 | 523.585 | 3.7 | 1894 |
| spread-100k | 100000 | 0 | 422.585 | 487.064 | 716.213 | 788.261 | 2.4 | 2112 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

