# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 85.815 | 146.431 | 228.978 | 301.799 | 11.7 | 1724 |
| mobs-25k | 25000 | 0 | 468.290 | 552.196 | 660.290 | 1015.589 | 2.1 | 1565 |
| mixed-150k | 150000 | 0 | 351.785 | 493.554 | 674.295 | 842.216 | 2.8 | 2306 |
| items-100k-mod | 100000 | 0 | 38.611 | 75.846 | 165.594 | 411.835 | 20.0 | 1742 |
| mixed-150k-mod | 150000 | 0 | 73.618 | 137.698 | 224.990 | 319.914 | 13.6 | 2410 |
| mobs-25k-mod | 25000 | 0 | 2585.898 | 2763.505 | 2903.171 | 2974.327 | 0.4 | 2004 |
| mobs-25k-native-mod | 25000 | 0 | 25.708 | 47.630 | 116.283 | 131.362 | 20.0 | 1832 |
| mobs-25k-native | 25000 | 0 | 403.730 | 464.574 | 595.465 | 692.285 | 2.5 | 1375 |
| spread-100k-mod | 100000 | 0 | 64.606 | 133.501 | 234.674 | 324.937 | 15.5 | 2236 |
| spread-100k | 100000 | 0 | 454.367 | 543.779 | 639.687 | 771.331 | 2.2 | 1887 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

