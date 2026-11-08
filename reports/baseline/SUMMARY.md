# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 76.553 | 128.646 | 197.606 | 306.596 | 13.1 | 1807 |
| mobs-25k | 25000 | 0 | 605.664 | 683.176 | 771.506 | 910.113 | 1.7 | 1730 |
| mixed-150k | 150000 | 0 | 507.537 | 678.937 | 908.435 | 1069.716 | 2.0 | 3212 |
| items-100k-mod | 100000 | 0 | 39.113 | 59.908 | 127.155 | 307.789 | 20.0 | 1415 |
| mixed-150k-mod | 150000 | 0 | 1639.690 | 1923.249 | 2119.448 | 2270.754 | 0.6 | 1887 |
| mobs-25k-mod | 25000 | 0 | 1556.971 | 1676.533 | 1775.256 | 1835.541 | 0.6 | 2182 |
| spread-100k-mod | 100000 | 0 | 982.622 | 1221.267 | 1443.011 | 1712.002 | 1.0 | 3001 |
| spread-100k | 100000 | 0 | 492.741 | 581.992 | 792.280 | 847.094 | 2.0 | 2641 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

