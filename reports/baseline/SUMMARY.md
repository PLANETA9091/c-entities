# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.691 | 90.344 | 154.045 | 219.603 | 15.0 | 1345 |
| mobs-25k | 25000 | 0 | 499.347 | 560.470 | 616.217 | 648.151 | 2.0 | 1556 |
| mixed-150k | 150000 | 0 | 455.918 | 554.762 | 770.236 | 861.667 | 2.2 | 2626 |
| items-100k-mod | 100000 | 0 | 25.653 | 64.334 | 126.131 | 161.858 | 20.0 | 1567 |
| mixed-150k-mod | 150000 | 0 | 214.494 | 308.812 | 444.955 | 514.889 | 4.7 | 2380 |
| mobs-25k-mod | 25000 | 0 | 263.321 | 461.783 | 579.327 | 633.443 | 3.8 | 1730 |
| spread-100k-mod | 100000 | 0 | 246.892 | 410.345 | 508.286 | 648.337 | 4.1 | 2286 |
| spread-100k | 100000 | 0 | 420.337 | 475.662 | 659.447 | 687.232 | 2.4 | 2068 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

