# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.851 | 81.351 | 143.950 | 197.576 | 15.9 | 930 |
| mobs-25k | 25000 | 0 | 352.087 | 395.614 | 439.486 | 469.333 | 2.8 | 1876 |
| mixed-150k | 150000 | 0 | 720.821 | 847.584 | 1335.326 | 1446.031 | 1.4 | 2103 |
| items-100k-mod | 100000 | 0 | 26.380 | 67.644 | 135.079 | 161.510 | 20.0 | 1381 |
| mixed-150k-mod | 150000 | 0 | 300.771 | 418.189 | 587.548 | 850.514 | 3.3 | 1755 |
| mobs-25k-mod | 25000 | 0 | 214.275 | 358.018 | 557.620 | 606.704 | 4.7 | 1620 |
| spread-100k-mod | 100000 | 0 | 328.808 | 475.717 | 610.121 | 788.889 | 3.0 | 1990 |
| spread-100k | 100000 | 0 | 651.403 | 742.813 | 1033.231 | 1203.186 | 1.5 | 2696 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

