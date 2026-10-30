# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.948 | 79.525 | 150.057 | 227.563 | 15.9 | 1428 |
| mobs-25k | 25000 | 0 | 559.938 | 624.568 | 670.767 | 717.785 | 1.8 | 1338 |
| mixed-150k | 150000 | 0 | 439.260 | 541.672 | 774.928 | 929.213 | 2.3 | 2427 |
| items-100k-mod | 100000 | 0 | 32.836 | 79.440 | 145.839 | 235.174 | 20.0 | 1595 |
| mixed-150k-mod | 150000 | 0 | 241.467 | 333.698 | 439.856 | 482.046 | 4.1 | 2097 |
| mobs-25k-mod | 25000 | 0 | 250.356 | 463.032 | 561.809 | 622.606 | 4.0 | 1627 |
| spread-100k-mod | 100000 | 0 | 258.915 | 375.582 | 458.613 | 630.615 | 3.9 | 1908 |
| spread-100k | 100000 | 0 | 442.567 | 510.931 | 733.533 | 779.200 | 2.3 | 2188 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

