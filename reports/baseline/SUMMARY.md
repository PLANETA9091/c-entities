# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 78.257 | 140.785 | 213.918 | 311.140 | 12.8 | 3168 |
| mobs-25k | 25000 | 0 | 326.599 | 398.254 | 471.454 | 585.145 | 3.1 | 1238 |
| mixed-150k | 150000 | 0 | 686.865 | 959.475 | 1523.527 | 1843.248 | 1.5 | 2031 |
| items-100k-mod | 100000 | 0 | 23.709 | 46.888 | 120.957 | 277.528 | 20.0 | 1699 |
| mixed-150k-mod | 150000 | 0 | 81.494 | 144.645 | 351.311 | 488.734 | 12.3 | 2261 |
| mobs-25k-mod | 25000 | 0 | 283.201 | 1246.317 | 2187.858 | 2296.185 | 3.5 | 1916 |
| mobs-25k-native-mod | 25000 | 0 | 21.899 | 45.523 | 109.966 | 243.483 | 20.0 | 1714 |
| mobs-25k-native | 25000 | 0 | 568.876 | 633.733 | 715.414 | 792.494 | 1.8 | 1869 |
| spread-100k-mod | 100000 | 0 | 54.578 | 109.373 | 216.742 | 278.843 | 18.3 | 2449 |
| spread-100k | 100000 | 0 | 434.714 | 550.794 | 697.714 | 834.445 | 2.3 | 2494 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

