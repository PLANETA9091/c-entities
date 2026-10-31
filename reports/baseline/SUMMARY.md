# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 72.232 | 92.274 | 165.241 | 222.506 | 13.8 | 827 |
| mobs-25k | 25000 | 0 | 418.111 | 459.743 | 497.960 | 548.728 | 2.4 | 1892 |
| mixed-150k | 150000 | 0 | 522.414 | 629.331 | 851.785 | 986.869 | 1.9 | 2669 |
| items-100k-mod | 100000 | 0 | 30.974 | 73.662 | 133.301 | 168.454 | 20.0 | 1385 |
| mixed-150k-mod | 150000 | 0 | 242.620 | 333.735 | 429.335 | 530.690 | 4.1 | 1660 |
| mobs-25k-mod | 25000 | 0 | 232.806 | 355.097 | 463.083 | 537.543 | 4.3 | 1333 |
| spread-100k-mod | 100000 | 0 | 237.084 | 359.115 | 490.457 | 629.830 | 4.2 | 1808 |
| spread-100k | 100000 | 0 | 345.554 | 416.552 | 579.812 | 674.845 | 2.9 | 2878 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

