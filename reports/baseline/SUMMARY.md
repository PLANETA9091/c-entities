# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.004 | 111.526 | 163.356 | 217.117 | 14.7 | 852 |
| mobs-25k | 25000 | 0 | 577.922 | 649.866 | 741.543 | 822.165 | 1.7 | 1508 |
| mixed-150k | 150000 | 0 | 596.741 | 801.291 | 1168.797 | 1313.508 | 1.7 | 1750 |
| items-100k-mod | 100000 | 0 | 27.093 | 42.962 | 122.648 | 213.591 | 20.0 | 1739 |
| mixed-150k-mod | 150000 | 0 | 181.544 | 357.438 | 540.454 | 645.752 | 5.5 | 2397 |
| mobs-25k-mod | 25000 | 0 | 2555.549 | 2746.751 | 2930.774 | 3025.815 | 0.4 | 1696 |
| mobs-25k-native-mod | 25000 | 0 | 22.133 | 40.665 | 116.903 | 198.534 | 20.0 | 1586 |
| mobs-25k-native | 25000 | 0 | 361.979 | 438.911 | 541.302 | 626.493 | 2.8 | 1043 |
| spread-100k-mod | 100000 | 0 | 161.255 | 242.268 | 400.962 | 504.171 | 6.2 | 2486 |
| spread-100k | 100000 | 0 | 400.579 | 504.878 | 643.611 | 702.356 | 2.5 | 1996 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

