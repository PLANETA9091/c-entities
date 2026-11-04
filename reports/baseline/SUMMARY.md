# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.338 | 87.326 | 154.653 | 204.132 | 14.4 | 2120 |
| mobs-25k | 25000 | 0 | 335.240 | 387.669 | 441.018 | 479.130 | 3.0 | 1924 |
| mixed-150k | 150000 | 0 | 594.969 | 707.328 | 950.376 | 1070.025 | 1.7 | 1777 |
| items-100k-mod | 100000 | 0 | 28.155 | 83.681 | 152.750 | 227.932 | 20.0 | 1605 |
| mixed-150k-mod | 150000 | 0 | 298.632 | 385.016 | 492.759 | 558.440 | 3.3 | 2517 |
| mobs-25k-mod | 25000 | 0 | 218.688 | 362.925 | 523.899 | 632.919 | 4.6 | 2016 |
| spread-100k-mod | 100000 | 0 | 442.985 | 555.047 | 722.985 | 984.199 | 2.3 | 3017 |
| spread-100k | 100000 | 0 | 577.779 | 681.569 | 981.139 | 1257.180 | 1.7 | 2178 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

