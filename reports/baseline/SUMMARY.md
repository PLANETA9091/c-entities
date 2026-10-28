# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 75.449 | 93.162 | 170.251 | 224.052 | 13.3 | 892 |
| mobs-25k | 25000 | 0 | 312.323 | 350.427 | 402.032 | 454.070 | 3.2 | 1669 |
| mixed-150k | 150000 | 0 | 726.749 | 898.387 | 1291.588 | 1827.926 | 1.4 | 2590 |
| items-100k-mod | 100000 | 0 | 37.846 | 94.705 | 180.983 | 251.314 | 20.0 | 1119 |
| mixed-150k-mod | 150000 | 0 | 544.210 | 713.605 | 969.045 | 1062.990 | 1.8 | 2075 |
| mobs-25k-mod | 25000 | 0 | 168.040 | 267.551 | 455.491 | 531.889 | 6.0 | 1850 |
| spread-100k-mod | 100000 | 0 | 200.483 | 356.007 | 538.007 | 722.955 | 5.0 | 2217 |
| spread-100k | 100000 | 0 | 526.666 | 622.176 | 871.263 | 1017.915 | 1.9 | 2028 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

