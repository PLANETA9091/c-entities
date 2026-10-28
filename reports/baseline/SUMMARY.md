# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.362 | 79.462 | 147.216 | 198.762 | 15.8 | 1315 |
| mobs-25k | 25000 | 0 | 509.435 | 563.617 | 615.061 | 658.766 | 2.0 | 2410 |
| mixed-150k | 150000 | 0 | 370.608 | 443.626 | 630.673 | 723.668 | 2.7 | 2261 |
| items-100k-mod | 100000 | 0 | 24.446 | 67.844 | 114.719 | 194.492 | 20.0 | 1313 |
| mixed-150k-mod | 150000 | 0 | 344.351 | 492.302 | 631.612 | 718.303 | 2.9 | 1856 |
| mobs-25k-mod | 25000 | 0 | 230.706 | 447.332 | 655.340 | 760.241 | 4.3 | 1312 |
| spread-100k-mod | 100000 | 0 | 157.711 | 241.244 | 339.523 | 410.430 | 6.3 | 2537 |
| spread-100k | 100000 | 0 | 304.796 | 366.805 | 547.943 | 606.173 | 3.3 | 1913 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

