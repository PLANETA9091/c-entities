# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 74.325 | 117.374 | 185.098 | 268.806 | 13.5 | 2518 |
| mobs-25k | 25000 | 0 | 743.615 | 832.186 | 910.380 | 1007.270 | 1.3 | 1136 |
| mixed-150k | 150000 | 0 | 558.421 | 809.735 | 1038.743 | 1356.954 | 1.8 | 1811 |
| items-100k-mod | 100000 | 0 | 16.386 | 29.713 | 101.401 | 259.984 | 20.0 | 1199 |
| mixed-150k-mod | 150000 | 0 | 61.826 | 109.069 | 255.882 | 430.417 | 16.2 | 2247 |
| mobs-25k-mod | 25000 | 0 | 3397.715 | 3558.367 | 3740.066 | 3822.569 | 0.3 | 1488 |
| mobs-25k-native-mod | 25000 | 0 | 21.096 | 45.752 | 119.533 | 227.960 | 20.0 | 1256 |
| mobs-25k-native | 25000 | 0 | 403.271 | 475.191 | 583.855 | 751.944 | 2.5 | 1693 |
| spread-100k-mod | 100000 | 0 | 54.479 | 96.312 | 223.213 | 375.298 | 18.4 | 1990 |
| spread-100k | 100000 | 0 | 484.836 | 582.918 | 691.435 | 834.747 | 2.1 | 1951 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

