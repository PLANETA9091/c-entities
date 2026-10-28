# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.969 | 87.161 | 179.774 | 254.914 | 15.2 | 1803 |
| mobs-25k | 25000 | 0 | 409.539 | 461.625 | 503.040 | 530.055 | 2.4 | 1170 |
| mixed-150k | 150000 | 0 | 500.658 | 628.324 | 927.330 | 1011.932 | 2.0 | 3028 |
| items-100k-mod | 100000 | 0 | 36.616 | 89.898 | 178.321 | 201.428 | 20.0 | 1103 |
| mixed-150k-mod | 150000 | 0 | 433.925 | 602.489 | 816.390 | 899.474 | 2.3 | 1768 |
| mobs-25k-mod | 25000 | 0 | 168.989 | 309.988 | 467.259 | 542.801 | 5.9 | 1376 |
| spread-100k-mod | 100000 | 0 | 161.048 | 251.389 | 370.985 | 473.686 | 6.2 | 1889 |
| spread-100k | 100000 | 0 | 301.428 | 367.482 | 532.515 | 608.232 | 3.3 | 2622 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

