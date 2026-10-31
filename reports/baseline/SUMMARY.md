# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.646 | 83.694 | 150.183 | 220.043 | 15.2 | 934 |
| mobs-25k | 25000 | 0 | 399.848 | 445.834 | 480.123 | 509.791 | 2.5 | 1707 |
| mixed-150k | 150000 | 0 | 626.623 | 770.279 | 1048.900 | 1130.409 | 1.6 | 3180 |
| items-100k-mod | 100000 | 0 | 32.884 | 92.840 | 140.592 | 208.033 | 20.0 | 1522 |
| mixed-150k-mod | 150000 | 0 | 259.398 | 372.575 | 506.590 | 615.277 | 3.9 | 1835 |
| mobs-25k-mod | 25000 | 0 | 223.102 | 326.258 | 467.804 | 560.436 | 4.5 | 1544 |
| spread-100k-mod | 100000 | 0 | 288.288 | 402.963 | 536.111 | 634.213 | 3.5 | 1961 |
| spread-100k | 100000 | 0 | 597.547 | 682.554 | 914.044 | 1001.202 | 1.7 | 2215 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

