# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.885 | 77.843 | 147.124 | 188.886 | 15.4 | 1372 |
| mobs-25k | 25000 | 0 | 249.237 | 284.696 | 317.919 | 346.289 | 4.0 | 1294 |
| mixed-150k | 150000 | 0 | 620.930 | 779.286 | 1069.976 | 1262.387 | 1.6 | 3108 |
| items-100k-mod | 100000 | 0 | 24.427 | 68.955 | 126.881 | 171.639 | 20.0 | 1148 |
| mixed-150k-mod | 150000 | 0 | 455.768 | 591.594 | 798.572 | 922.441 | 2.2 | 1830 |
| mobs-25k-mod | 25000 | 0 | 152.138 | 237.428 | 359.440 | 418.845 | 6.6 | 1378 |
| spread-100k-mod | 100000 | 0 | 163.069 | 285.153 | 429.505 | 548.120 | 6.1 | 2768 |
| spread-100k | 100000 | 0 | 365.870 | 428.692 | 646.907 | 680.848 | 2.7 | 1898 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

