# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 85.786 | 113.656 | 144.212 | 201.734 | 11.7 | 857 |
| mobs-25k | 25000 | 0 | 368.768 | 409.931 | 448.004 | 474.508 | 2.7 | 1582 |
| mixed-150k | 150000 | 0 | 789.062 | 840.353 | 892.159 | 950.885 | 1.3 | 3102 |
| items-100k-mod | 100000 | 0 | 102.156 | 108.412 | 120.376 | 162.360 | 9.8 | 1036 |
| mixed-150k-mod | 150000 | 0 | 688.630 | 746.255 | 796.543 | 867.764 | 1.5 | 1879 |
| mobs-25k-mod | 25000 | 0 | 350.069 | 396.564 | 448.890 | 599.677 | 2.9 | 1575 |
| spread-100k-mod | 100000 | 0 | 380.754 | 452.990 | 519.056 | 703.253 | 2.6 | 2475 |
| spread-100k | 100000 | 0 | 371.160 | 438.984 | 525.641 | 675.692 | 2.7 | 2329 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

