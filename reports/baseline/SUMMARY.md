# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.377 | 87.529 | 157.583 | 250.680 | 14.4 | 1078 |
| mobs-25k | 25000 | 0 | 503.716 | 567.782 | 617.840 | 650.131 | 2.0 | 1264 |
| mixed-150k | 150000 | 0 | 661.967 | 806.419 | 1088.532 | 1159.936 | 1.5 | 2282 |
| items-100k-mod | 100000 | 0 | 33.211 | 93.150 | 158.475 | 403.348 | 20.0 | 1264 |
| mixed-150k-mod | 150000 | 0 | 369.468 | 466.819 | 677.624 | 755.453 | 2.7 | 1718 |
| mobs-25k-mod | 25000 | 0 | 207.272 | 349.817 | 524.545 | 591.694 | 4.8 | 2428 |
| spread-100k-mod | 100000 | 0 | 205.712 | 351.791 | 566.017 | 770.206 | 4.9 | 2373 |
| spread-100k | 100000 | 0 | 533.938 | 605.296 | 863.625 | 956.753 | 1.9 | 2149 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

