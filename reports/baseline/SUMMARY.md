# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.342 | 79.233 | 152.422 | 195.526 | 15.5 | 1749 |
| mobs-25k | 25000 | 0 | 375.227 | 418.954 | 459.032 | 491.336 | 2.7 | 1337 |
| mixed-150k | 150000 | 0 | 537.846 | 653.049 | 933.239 | 1066.755 | 1.9 | 2137 |
| items-100k-mod | 100000 | 0 | 10.793 | 13.763 | 42.030 | 92.222 | 20.0 | 1361 |
| mixed-150k-mod | 150000 | 0 | 289.555 | 360.787 | 419.858 | 445.125 | 3.5 | 2323 |
| mobs-25k-mod | 25000 | 0 | 184.813 | 276.258 | 362.057 | 393.498 | 5.4 | 1470 |
| spread-100k-mod | 100000 | 0 | 145.246 | 220.144 | 305.722 | 423.297 | 6.9 | 2763 |
| spread-100k | 100000 | 0 | 402.921 | 463.885 | 659.083 | 687.934 | 2.5 | 2091 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

