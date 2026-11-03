# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.990 | 79.544 | 150.903 | 192.842 | 15.6 | 1361 |
| mobs-25k | 25000 | 0 | 522.107 | 587.330 | 641.617 | 701.449 | 1.9 | 998 |
| mixed-150k | 150000 | 0 | 449.686 | 574.369 | 794.020 | 939.548 | 2.2 | 2737 |
| items-100k-mod | 100000 | 0 | 26.162 | 58.871 | 154.146 | 210.364 | 20.0 | 1649 |
| mixed-150k-mod | 150000 | 0 | 312.158 | 403.991 | 529.817 | 573.868 | 3.2 | 1848 |
| mobs-25k-mod | 25000 | 0 | 350.691 | 590.639 | 721.115 | 772.867 | 2.9 | 1823 |
| spread-100k-mod | 100000 | 0 | 430.412 | 563.525 | 705.757 | 905.918 | 2.3 | 2777 |
| spread-100k | 100000 | 0 | 511.229 | 582.031 | 816.247 | 884.270 | 2.0 | 2116 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

