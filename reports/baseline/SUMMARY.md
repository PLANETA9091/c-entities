# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 75.807 | 132.864 | 204.603 | 258.835 | 13.2 | 1580 |
| mobs-25k | 25000 | 0 | 515.480 | 595.200 | 671.218 | 850.597 | 1.9 | 1139 |
| mixed-150k | 150000 | 0 | 464.033 | 673.536 | 854.518 | 952.574 | 2.2 | 3069 |
| items-100k-mod | 100000 | 0 | 32.340 | 60.356 | 129.741 | 286.082 | 20.0 | 1742 |
| mixed-150k-mod | 150000 | 0 | 49.710 | 76.054 | 176.832 | 251.063 | 20.0 | 2408 |
| mobs-25k-mod | 25000 | 0 | 2105.196 | 2215.313 | 2354.489 | 2391.421 | 0.5 | 2514 |
| mobs-25k-native-mod | 25000 | 0 | 27.440 | 55.797 | 126.154 | 247.499 | 20.0 | 1649 |
| mobs-25k-native | 25000 | 0 | 561.101 | 632.876 | 705.341 | 899.647 | 1.8 | 1847 |
| spread-100k-mod | 100000 | 0 | 52.251 | 78.763 | 210.200 | 365.049 | 19.1 | 2201 |
| spread-100k | 100000 | 0 | 288.754 | 367.086 | 459.660 | 513.915 | 3.5 | 2404 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

