# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.956 | 125.810 | 191.247 | 272.787 | 14.5 | 1107 |
| mobs-25k | 25000 | 0 | 500.593 | 577.239 | 703.131 | 959.494 | 2.0 | 1342 |
| mixed-150k | 150000 | 0 | 770.099 | 1080.606 | 1404.354 | 1607.466 | 1.3 | 2735 |
| items-100k-mod | 100000 | 0 | 32.275 | 59.886 | 128.879 | 288.245 | 20.0 | 1792 |
| mixed-150k-mod | 150000 | 0 | 95.787 | 165.776 | 286.345 | 371.573 | 10.4 | 2065 |
| mobs-25k-mod | 25000 | 0 | 2417.572 | 2585.495 | 2770.637 | 2933.289 | 0.4 | 2194 |
| mobs-25k-native-mod | 25000 | 0 | 24.967 | 42.511 | 111.367 | 192.332 | 20.0 | 1744 |
| mobs-25k-native | 25000 | 0 | 428.401 | 517.688 | 653.100 | 725.928 | 2.3 | 1631 |
| spread-100k-mod | 100000 | 0 | 60.199 | 116.687 | 208.383 | 247.267 | 16.6 | 2398 |
| spread-100k | 100000 | 0 | 381.933 | 478.003 | 582.409 | 658.008 | 2.6 | 1916 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

