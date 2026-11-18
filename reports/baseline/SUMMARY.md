# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 85.293 | 153.330 | 230.857 | 266.863 | 11.7 | 1322 |
| mobs-25k | 25000 | 0 | 411.550 | 496.349 | 599.478 | 751.264 | 2.4 | 958 |
| mixed-150k | 150000 | 0 | 596.340 | 806.913 | 1046.494 | 1233.742 | 1.7 | 1789 |
| items-100k-mod | 100000 | 0 | 22.367 | 47.887 | 141.378 | 225.653 | 20.0 | 1841 |
| mixed-150k-mod | 150000 | 0 | 75.438 | 139.591 | 281.771 | 527.074 | 13.3 | 2193 |
| mobs-25k-mod | 25000 | 0 | 417.495 | 2731.550 | 2870.113 | 3168.453 | 2.4 | 2191 |
| mobs-25k-native-mod | 25000 | 0 | 26.162 | 56.330 | 133.425 | 236.649 | 20.0 | 1675 |
| mobs-25k-native | 25000 | 0 | 492.275 | 573.725 | 700.432 | 856.103 | 2.0 | 975 |
| spread-100k-mod | 100000 | 0 | 40.824 | 64.013 | 174.598 | 225.590 | 20.0 | 2420 |
| spread-100k | 100000 | 0 | 397.347 | 498.437 | 633.183 | 725.064 | 2.5 | 2452 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

