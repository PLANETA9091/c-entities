# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.307 | 91.047 | 162.038 | 200.469 | 13.8 | 1660 |
| mobs-25k | 25000 | 0 | 535.793 | 596.459 | 657.654 | 685.910 | 1.9 | 1622 |
| mixed-150k | 150000 | 0 | 667.300 | 824.396 | 1244.175 | 1392.837 | 1.5 | 1798 |
| items-100k-mod | 100000 | 0 | 32.967 | 94.824 | 163.413 | 193.156 | 20.0 | 1362 |
| mixed-150k-mod | 150000 | 0 | 528.205 | 644.961 | 850.134 | 946.434 | 1.9 | 1871 |
| mobs-25k-mod | 25000 | 0 | 216.433 | 381.934 | 542.926 | 635.290 | 4.6 | 1874 |
| spread-100k-mod | 100000 | 0 | 191.572 | 329.869 | 444.858 | 633.248 | 5.2 | 2132 |
| spread-100k | 100000 | 0 | 396.856 | 474.041 | 678.703 | 761.871 | 2.5 | 2192 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

