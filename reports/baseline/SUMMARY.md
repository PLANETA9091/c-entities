# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.820 | 134.825 | 198.780 | 267.470 | 14.1 | 1638 |
| mobs-25k | 25000 | 0 | 540.133 | 616.464 | 691.097 | 758.881 | 1.9 | 1177 |
| mixed-150k | 150000 | 0 | 497.212 | 695.654 | 896.003 | 1124.052 | 2.0 | 1611 |
| items-100k-mod | 100000 | 0 | 16.990 | 35.069 | 110.351 | 226.717 | 20.0 | 1406 |
| mixed-150k-mod | 150000 | 0 | 57.822 | 99.539 | 230.496 | 299.683 | 17.3 | 1968 |
| mobs-25k-mod | 25000 | 0 | 3048.575 | 3160.234 | 3322.731 | 3389.471 | 0.3 | 1430 |
| mobs-25k-native-mod | 25000 | 0 | 18.844 | 34.546 | 113.866 | 192.006 | 20.0 | 1331 |
| mobs-25k-native | 25000 | 0 | 406.853 | 486.831 | 595.348 | 719.067 | 2.5 | 1460 |
| spread-100k-mod | 100000 | 0 | 42.144 | 64.396 | 190.250 | 253.082 | 20.0 | 2111 |
| spread-100k | 100000 | 0 | 419.839 | 508.607 | 606.378 | 689.427 | 2.4 | 1793 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

