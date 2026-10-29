# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.652 | 88.022 | 164.628 | 215.569 | 14.4 | 1954 |
| mobs-25k | 25000 | 0 | 491.453 | 551.440 | 606.383 | 760.550 | 2.0 | 916 |
| mixed-150k | 150000 | 0 | 665.572 | 796.152 | 1019.281 | 1161.592 | 1.5 | 3198 |
| items-100k-mod | 100000 | 0 | 29.964 | 85.342 | 141.623 | 252.213 | 20.0 | 1177 |
| mixed-150k-mod | 150000 | 0 | 326.102 | 416.430 | 546.909 | 610.549 | 3.1 | 1714 |
| mobs-25k-mod | 25000 | 0 | 178.505 | 371.654 | 644.804 | 809.211 | 5.6 | 1939 |
| spread-100k-mod | 100000 | 0 | 175.912 | 312.737 | 457.438 | 621.321 | 5.7 | 2559 |
| spread-100k | 100000 | 0 | 512.741 | 608.461 | 844.667 | 953.744 | 2.0 | 2226 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

