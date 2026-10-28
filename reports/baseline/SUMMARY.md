# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.708 | 81.086 | 145.993 | 194.931 | 15.5 | 1066 |
| mobs-25k | 25000 | 0 | 416.910 | 463.647 | 506.723 | 550.712 | 2.4 | 927 |
| mixed-150k | 150000 | 0 | 634.121 | 754.179 | 968.076 | 1086.821 | 1.6 | 1569 |
| items-100k-mod | 100000 | 0 | 25.358 | 57.587 | 137.065 | 246.532 | 20.0 | 1363 |
| mixed-150k-mod | 150000 | 0 | 421.779 | 561.474 | 719.347 | 802.715 | 2.4 | 1835 |
| mobs-25k-mod | 25000 | 0 | 181.156 | 312.430 | 524.162 | 577.413 | 5.5 | 1782 |
| spread-100k-mod | 100000 | 0 | 199.866 | 366.014 | 578.022 | 777.692 | 5.0 | 1925 |
| spread-100k | 100000 | 0 | 536.111 | 609.237 | 899.546 | 1030.308 | 1.9 | 2207 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

