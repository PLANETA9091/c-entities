# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.427 | 126.030 | 195.396 | 306.332 | 14.6 | 1833 |
| mobs-25k | 25000 | 0 | 406.687 | 468.350 | 557.690 | 693.089 | 2.5 | 995 |
| mixed-150k | 150000 | 0 | 555.568 | 769.031 | 998.440 | 1106.677 | 1.8 | 2070 |
| items-100k-mod | 100000 | 0 | 33.367 | 58.309 | 129.071 | 338.849 | 20.0 | 1665 |
| mixed-150k-mod | 150000 | 0 | 84.208 | 133.966 | 286.836 | 392.010 | 11.9 | 2089 |
| mobs-25k-mod | 25000 | 0 | 2063.227 | 2181.921 | 2302.556 | 2353.673 | 0.5 | 2379 |
| mobs-25k-native-mod | 25000 | 0 | 26.974 | 48.713 | 128.486 | 256.000 | 20.0 | 1729 |
| mobs-25k-native | 25000 | 0 | 419.535 | 493.451 | 611.284 | 728.631 | 2.4 | 1667 |
| spread-100k-mod | 100000 | 0 | 52.697 | 80.824 | 181.333 | 203.518 | 19.0 | 2218 |
| spread-100k | 100000 | 0 | 379.318 | 460.707 | 559.071 | 633.279 | 2.6 | 2171 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

