# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 58.740 | 75.611 | 130.839 | 191.906 | 17.0 | 1367 |
| mobs-25k | 25000 | 0 | 500.506 | 564.027 | 614.484 | 666.081 | 2.0 | 2187 |
| mixed-150k | 150000 | 0 | 528.331 | 640.900 | 891.284 | 1035.211 | 1.9 | 3079 |
| items-100k-mod | 100000 | 0 | 25.188 | 69.004 | 124.067 | 190.868 | 20.0 | 1518 |
| mixed-150k-mod | 150000 | 0 | 428.058 | 569.739 | 777.192 | 830.982 | 2.3 | 1730 |
| mobs-25k-mod | 25000 | 0 | 238.683 | 422.147 | 654.669 | 725.181 | 4.2 | 1536 |
| spread-100k-mod | 100000 | 0 | 204.220 | 376.417 | 520.013 | 720.635 | 4.9 | 2064 |
| spread-100k | 100000 | 0 | 552.840 | 629.989 | 884.973 | 968.569 | 1.8 | 2106 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

