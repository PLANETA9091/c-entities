# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.493 | 82.668 | 156.656 | 250.952 | 15.3 | 984 |
| mobs-25k | 25000 | 0 | 332.959 | 379.549 | 414.071 | 446.315 | 3.0 | 1238 |
| mixed-150k | 150000 | 0 | 646.791 | 784.733 | 1129.199 | 1291.562 | 1.5 | 2891 |
| items-100k-mod | 100000 | 0 | 33.636 | 85.648 | 142.103 | 213.452 | 20.0 | 1545 |
| mixed-150k-mod | 150000 | 0 | 393.098 | 493.211 | 653.900 | 734.802 | 2.5 | 1836 |
| mobs-25k-mod | 25000 | 0 | 176.383 | 320.755 | 530.198 | 574.456 | 5.7 | 1488 |
| spread-100k-mod | 100000 | 0 | 170.496 | 280.681 | 405.109 | 550.672 | 5.9 | 2664 |
| spread-100k | 100000 | 0 | 380.289 | 437.131 | 641.511 | 761.155 | 2.6 | 1993 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

