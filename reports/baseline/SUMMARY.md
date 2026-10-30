# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.202 | 81.230 | 146.841 | 203.530 | 15.3 | 1195 |
| mobs-25k | 25000 | 0 | 530.864 | 583.829 | 635.853 | 682.171 | 1.9 | 947 |
| mixed-150k | 150000 | 0 | 548.180 | 655.604 | 916.334 | 1044.579 | 1.8 | 2773 |
| items-100k-mod | 100000 | 0 | 25.972 | 72.514 | 130.333 | 191.428 | 20.0 | 1188 |
| mixed-150k-mod | 150000 | 0 | 261.390 | 367.056 | 464.910 | 666.362 | 3.8 | 1770 |
| mobs-25k-mod | 25000 | 0 | 258.955 | 525.848 | 674.009 | 783.775 | 3.9 | 1665 |
| spread-100k-mod | 100000 | 0 | 289.203 | 472.705 | 621.785 | 841.465 | 3.5 | 1964 |
| spread-100k | 100000 | 0 | 522.283 | 609.046 | 850.331 | 982.951 | 1.9 | 2064 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

