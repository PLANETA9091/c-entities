# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.078 | 78.019 | 144.363 | 185.746 | 16.1 | 1724 |
| mobs-25k | 25000 | 0 | 541.027 | 604.155 | 682.703 | 751.028 | 1.8 | 1570 |
| mixed-150k | 150000 | 0 | 870.315 | 1070.382 | 1428.180 | 1601.550 | 1.1 | 2401 |
| items-100k-mod | 100000 | 0 | 32.655 | 92.916 | 162.703 | 228.944 | 20.0 | 1373 |
| mixed-150k-mod | 150000 | 0 | 269.418 | 376.464 | 483.027 | 533.882 | 3.7 | 1840 |
| mobs-25k-mod | 25000 | 0 | 315.199 | 522.422 | 643.634 | 859.731 | 3.2 | 1604 |
| spread-100k-mod | 100000 | 0 | 308.387 | 476.792 | 567.259 | 741.058 | 3.2 | 2016 |
| spread-100k | 100000 | 0 | 535.316 | 604.402 | 847.868 | 967.867 | 1.9 | 2027 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

