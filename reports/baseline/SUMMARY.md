# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 57.165 | 71.737 | 129.797 | 187.106 | 17.5 | 1644 |
| mobs-25k | 25000 | 0 | 505.337 | 561.646 | 618.679 | 643.126 | 2.0 | 1274 |
| mixed-150k | 150000 | 0 | 521.930 | 644.933 | 856.734 | 1002.274 | 1.9 | 2719 |
| items-100k-mod | 100000 | 0 | 12.246 | 16.799 | 67.657 | 92.898 | 20.0 | 1442 |
| mixed-150k-mod | 150000 | 0 | 307.830 | 398.843 | 534.899 | 628.177 | 3.2 | 1822 |
| mobs-25k-mod | 25000 | 0 | 226.372 | 403.600 | 616.185 | 762.914 | 4.4 | 1402 |
| spread-100k-mod | 100000 | 0 | 170.003 | 290.211 | 468.608 | 521.578 | 5.9 | 2316 |
| spread-100k | 100000 | 0 | 401.338 | 465.186 | 675.594 | 722.526 | 2.5 | 2143 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

