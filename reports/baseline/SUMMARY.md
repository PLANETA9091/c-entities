# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.629 | 88.632 | 158.208 | 209.850 | 14.6 | 1982 |
| mobs-25k | 25000 | 0 | 537.939 | 599.967 | 641.801 | 675.729 | 1.9 | 2193 |
| mixed-150k | 150000 | 0 | 444.993 | 541.421 | 721.540 | 860.227 | 2.2 | 2530 |
| items-100k-mod | 100000 | 0 | 33.068 | 85.823 | 136.424 | 216.255 | 20.0 | 1491 |
| mixed-150k-mod | 150000 | 0 | 246.647 | 338.537 | 447.273 | 535.165 | 4.1 | 1761 |
| mobs-25k-mod | 25000 | 0 | 280.513 | 462.095 | 606.934 | 679.517 | 3.6 | 1456 |
| spread-100k-mod | 100000 | 0 | 280.885 | 436.553 | 573.075 | 763.040 | 3.6 | 2106 |
| spread-100k | 100000 | 0 | 505.313 | 573.408 | 791.211 | 839.112 | 2.0 | 2140 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

