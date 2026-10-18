# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| control | 0 | 0 | 0.158 | 0.267 | 0.369 | 4.868 | 20.0 | 575 |
| items-25k | 25000 | 0 | 19.010 | 21.364 | 36.314 | 44.597 | 20.0 | 723 |
| items-100k | 100000 | 0 | 65.825 | 72.598 | 107.992 | 153.920 | 15.2 | 1401 |
| mobs-10k | 10000 | 0 | 100.256 | 107.747 | 142.936 | 156.934 | 10.0 | 1421 |
| mobs-25k | 25000 | 0 | 488.894 | 522.370 | 568.492 | 589.187 | 2.0 | 1334 |
| mixed-50k | 50000 | 0 | 110.124 | 123.001 | 153.875 | 182.340 | 9.1 | 1806 |
| mixed-150k | 150000 | 0 | 909.686 | 1014.156 | 1082.055 | 1185.789 | 1.1 | 1642 |
| tiles-25k | 0 | 24964 | 6.971 | 8.107 | 10.897 | 21.573 | 20.0 | 613 |
| tiles-100k | 0 | 68121 | 27.477 | 30.137 | 39.633 | 53.953 | 20.0 | 683 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

