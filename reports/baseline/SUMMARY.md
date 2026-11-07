# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.918 | 85.685 | 150.085 | 189.966 | 14.3 | 1007 |
| mobs-25k | 25000 | 0 | 507.030 | 562.115 | 617.243 | 627.535 | 2.0 | 1524 |
| mixed-150k | 150000 | 0 | 675.169 | 857.212 | 1192.543 | 1356.624 | 1.5 | 2406 |
| items-100k-mod | 100000 | 0 | 30.152 | 80.585 | 156.592 | 268.083 | 20.0 | 1729 |
| mixed-150k-mod | 150000 | 0 | 419.796 | 506.013 | 748.116 | 833.734 | 2.4 | 2029 |
| mobs-25k-mod | 25000 | 0 | 376.105 | 616.661 | 772.098 | 859.847 | 2.7 | 1863 |
| spread-100k-mod | 100000 | 0 | 405.061 | 535.138 | 714.332 | 930.484 | 2.5 | 2058 |
| spread-100k | 100000 | 0 | 529.875 | 597.347 | 831.263 | 892.592 | 1.9 | 2194 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

