# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.309 | 80.633 | 156.291 | 227.850 | 15.3 | 1001 |
| mobs-25k | 25000 | 0 | 531.934 | 595.071 | 656.379 | 719.416 | 1.9 | 1244 |
| mixed-150k | 150000 | 0 | 731.863 | 939.789 | 1470.307 | 1636.789 | 1.4 | 2825 |
| items-100k-mod | 100000 | 0 | 34.793 | 80.037 | 156.116 | 213.206 | 20.0 | 1458 |
| mixed-150k-mod | 150000 | 0 | 312.385 | 437.885 | 634.225 | 778.484 | 3.2 | 1637 |
| mobs-25k-mod | 25000 | 0 | 288.060 | 475.494 | 584.071 | 642.926 | 3.5 | 1394 |
| spread-100k-mod | 100000 | 0 | 236.115 | 363.576 | 475.504 | 636.387 | 4.2 | 2736 |
| spread-100k | 100000 | 0 | 341.723 | 400.359 | 584.371 | 601.955 | 2.9 | 2686 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

