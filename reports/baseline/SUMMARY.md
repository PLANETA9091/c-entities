# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.991 | 84.103 | 144.375 | 193.442 | 15.2 | 1287 |
| mobs-25k | 25000 | 0 | 495.853 | 567.561 | 632.713 | 714.873 | 2.0 | 1894 |
| mixed-150k | 150000 | 0 | 630.300 | 770.253 | 1084.440 | 1172.731 | 1.6 | 3326 |
| items-100k-mod | 100000 | 0 | 31.343 | 76.257 | 136.588 | 203.992 | 20.0 | 1423 |
| mixed-150k-mod | 150000 | 0 | 372.771 | 495.360 | 697.421 | 775.689 | 2.7 | 1922 |
| mobs-25k-mod | 25000 | 0 | 316.525 | 557.130 | 664.121 | 796.179 | 3.2 | 1917 |
| spread-100k-mod | 100000 | 0 | 340.535 | 457.493 | 577.419 | 783.639 | 2.9 | 2239 |
| spread-100k | 100000 | 0 | 426.384 | 485.619 | 681.370 | 722.935 | 2.3 | 2153 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

