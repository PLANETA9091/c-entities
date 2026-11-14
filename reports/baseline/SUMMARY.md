# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.757 | 115.617 | 173.359 | 230.418 | 13.7 | 1372 |
| mobs-25k | 25000 | 0 | 490.949 | 560.093 | 648.119 | 820.065 | 2.0 | 1074 |
| mixed-150k | 150000 | 0 | 369.108 | 529.052 | 709.924 | 902.639 | 2.7 | 2372 |
| items-100k-mod | 100000 | 0 | 32.425 | 58.615 | 135.624 | 228.431 | 20.0 | 1628 |
| mixed-150k-mod | 150000 | 0 | 152.955 | 254.976 | 398.155 | 498.873 | 6.5 | 2352 |
| mobs-25k-mod | 25000 | 0 | 2357.552 | 2537.066 | 2658.730 | 2741.012 | 0.4 | 2061 |
| mobs-25k-native-mod | 25000 | 0 | 23.871 | 44.124 | 115.937 | 170.117 | 20.0 | 1638 |
| mobs-25k-native | 25000 | 0 | 477.977 | 550.815 | 643.695 | 713.284 | 2.1 | 1582 |
| spread-100k-mod | 100000 | 0 | 139.564 | 194.604 | 325.821 | 369.491 | 7.2 | 2851 |
| spread-100k | 100000 | 0 | 419.172 | 515.250 | 633.377 | 712.286 | 2.4 | 1684 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

