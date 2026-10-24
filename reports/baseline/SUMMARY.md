# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.732 | 77.089 | 142.020 | 189.356 | 15.7 | 793 |
| mobs-25k | 25000 | 0 | 271.988 | 314.924 | 358.084 | 389.418 | 3.7 | 1651 |
| mixed-150k | 150000 | 0 | 435.162 | 523.257 | 724.464 | 853.230 | 2.3 | 3000 |
| items-100k-mod | 100000 | 0 | 32.631 | 55.642 | 156.013 | 231.683 | 20.0 | 1167 |
| mixed-150k-mod | 150000 | 0 | 443.038 | 564.219 | 831.286 | 1241.839 | 2.3 | 3138 |
| mobs-25k-mod | 25000 | 0 | 266.043 | 317.905 | 389.585 | 433.677 | 3.8 | 1506 |
| spread-100k-mod | 100000 | 0 | 419.388 | 486.573 | 710.201 | 758.785 | 2.4 | 2313 |
| spread-100k | 100000 | 0 | 396.707 | 456.522 | 651.100 | 729.510 | 2.5 | 2004 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

