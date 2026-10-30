# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.895 | 86.227 | 153.606 | 207.049 | 14.9 | 1583 |
| mobs-25k | 25000 | 0 | 512.186 | 567.183 | 613.498 | 656.898 | 2.0 | 915 |
| mixed-150k | 150000 | 0 | 711.056 | 874.543 | 1275.650 | 1370.885 | 1.4 | 2307 |
| items-100k-mod | 100000 | 0 | 27.519 | 72.028 | 161.167 | 204.118 | 20.0 | 1769 |
| mixed-150k-mod | 150000 | 0 | 267.100 | 398.873 | 502.366 | 619.109 | 3.7 | 1770 |
| mobs-25k-mod | 25000 | 0 | 264.276 | 439.787 | 560.752 | 629.711 | 3.8 | 1502 |
| spread-100k-mod | 100000 | 0 | 273.077 | 414.122 | 506.441 | 591.730 | 3.7 | 2040 |
| spread-100k | 100000 | 0 | 490.723 | 559.141 | 778.810 | 825.222 | 2.0 | 2151 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

