# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 80.925 | 140.016 | 206.756 | 257.309 | 12.4 | 1616 |
| mobs-25k | 25000 | 0 | 414.364 | 488.047 | 604.796 | 723.163 | 2.4 | 1863 |
| mixed-150k | 150000 | 0 | 606.361 | 894.764 | 1263.667 | 1469.254 | 1.6 | 2175 |
| items-100k-mod | 100000 | 0 | 33.218 | 51.264 | 130.816 | 245.314 | 20.0 | 1548 |
| mixed-150k-mod | 150000 | 0 | 168.082 | 289.521 | 414.766 | 488.572 | 5.9 | 2234 |
| mobs-25k-mod | 25000 | 0 | 2256.342 | 2405.203 | 2621.107 | 2673.514 | 0.4 | 1716 |
| mobs-25k-native-mod | 25000 | 0 | 40.087 | 73.707 | 194.117 | 212.027 | 20.0 | 1532 |
| mobs-25k-native | 25000 | 0 | 400.821 | 467.631 | 592.052 | 716.372 | 2.5 | 1874 |
| spread-100k-mod | 100000 | 0 | 191.408 | 269.143 | 372.899 | 566.088 | 5.2 | 1993 |
| spread-100k | 100000 | 0 | 375.066 | 467.873 | 575.797 | 642.521 | 2.7 | 1785 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

