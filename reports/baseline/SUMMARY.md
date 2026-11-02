# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.316 | 79.170 | 148.764 | 218.725 | 15.5 | 1236 |
| mobs-25k | 25000 | 0 | 526.974 | 583.336 | 625.800 | 674.718 | 1.9 | 1231 |
| mixed-150k | 150000 | 0 | 653.048 | 848.158 | 1062.716 | 1308.111 | 1.5 | 1826 |
| items-100k-mod | 100000 | 0 | 27.218 | 69.552 | 159.995 | 173.470 | 20.0 | 1681 |
| mixed-150k-mod | 150000 | 0 | 267.246 | 377.106 | 567.209 | 675.695 | 3.7 | 1961 |
| mobs-25k-mod | 25000 | 0 | 291.546 | 519.508 | 666.751 | 948.736 | 3.4 | 1824 |
| spread-100k-mod | 100000 | 0 | 311.157 | 512.162 | 668.542 | 885.825 | 3.2 | 2400 |
| spread-100k | 100000 | 0 | 562.695 | 688.200 | 957.350 | 1067.124 | 1.8 | 2216 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

