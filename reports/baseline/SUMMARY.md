# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.596 | 85.667 | 150.408 | 212.356 | 15.0 | 804 |
| mobs-25k | 25000 | 0 | 358.830 | 401.820 | 436.127 | 496.054 | 2.8 | 1278 |
| mixed-150k | 150000 | 0 | 490.076 | 595.641 | 854.017 | 937.817 | 2.0 | 3092 |
| items-100k-mod | 100000 | 0 | 41.354 | 65.287 | 155.074 | 350.486 | 20.0 | 1231 |
| mixed-150k-mod | 150000 | 0 | 321.187 | 399.958 | 441.505 | 519.888 | 3.1 | 2093 |
| mobs-25k-mod | 25000 | 0 | 149.904 | 292.603 | 372.073 | 407.335 | 6.7 | 1427 |
| spread-100k-mod | 100000 | 0 | 178.672 | 283.340 | 393.372 | 510.087 | 5.6 | 1987 |
| spread-100k | 100000 | 0 | 425.620 | 503.785 | 712.694 | 816.685 | 2.3 | 2284 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

