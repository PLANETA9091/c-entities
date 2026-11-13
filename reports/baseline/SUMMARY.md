# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.101 | 133.508 | 195.524 | 260.785 | 13.9 | 2583 |
| mobs-25k | 25000 | 0 | 485.409 | 562.808 | 644.669 | 723.101 | 2.1 | 925 |
| mixed-150k | 150000 | 0 | 549.413 | 743.263 | 961.438 | 1040.476 | 1.8 | 2191 |
| items-100k-mod | 100000 | 0 | 33.856 | 58.677 | 142.020 | 379.823 | 20.0 | 1783 |
| mixed-150k-mod | 150000 | 0 | 166.132 | 264.391 | 367.160 | 451.762 | 6.0 | 2100 |
| mobs-25k-mod | 25000 | 0 | 1958.564 | 2090.761 | 2199.885 | 2277.140 | 0.5 | 1555 |
| mobs-25k-native-mod | 25000 | 0 | 24.795 | 42.431 | 127.059 | 219.130 | 20.0 | 1639 |
| mobs-25k-native | 25000 | 0 | 489.987 | 562.934 | 672.409 | 788.664 | 2.0 | 1297 |
| spread-100k-mod | 100000 | 0 | 138.978 | 194.925 | 289.588 | 317.051 | 7.2 | 2735 |
| spread-100k | 100000 | 0 | 370.631 | 475.068 | 593.915 | 680.740 | 2.7 | 1915 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

