# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.660 | 84.342 | 150.225 | 203.796 | 16.0 | 2322 |
| mobs-25k | 25000 | 0 | 499.690 | 550.914 | 600.710 | 642.529 | 2.0 | 1345 |
| mixed-150k | 150000 | 0 | 655.685 | 783.468 | 1101.673 | 1227.320 | 1.5 | 1761 |
| items-100k-mod | 100000 | 0 | 27.972 | 79.099 | 169.639 | 251.951 | 20.0 | 1501 |
| mixed-150k-mod | 150000 | 0 | 259.109 | 360.748 | 492.575 | 582.217 | 3.9 | 1907 |
| mobs-25k-mod | 25000 | 0 | 285.213 | 466.678 | 572.845 | 627.060 | 3.5 | 1978 |
| spread-100k-mod | 100000 | 0 | 220.220 | 328.686 | 417.117 | 528.108 | 4.5 | 2435 |
| spread-100k | 100000 | 0 | 380.922 | 443.071 | 631.224 | 729.997 | 2.6 | 2009 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

