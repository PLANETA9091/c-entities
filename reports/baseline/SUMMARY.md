# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.923 | 80.280 | 145.470 | 179.197 | 15.4 | 1590 |
| mobs-25k | 25000 | 0 | 402.201 | 449.867 | 493.486 | 544.027 | 2.5 | 1185 |
| mixed-150k | 150000 | 0 | 716.186 | 906.299 | 1354.557 | 1550.205 | 1.4 | 2456 |
| items-100k-mod | 100000 | 0 | 29.132 | 87.211 | 153.158 | 392.269 | 20.0 | 1186 |
| mixed-150k-mod | 150000 | 0 | 488.916 | 648.213 | 851.938 | 994.292 | 2.0 | 1737 |
| mobs-25k-mod | 25000 | 0 | 239.448 | 469.643 | 668.124 | 798.936 | 4.2 | 1590 |
| spread-100k-mod | 100000 | 0 | 159.615 | 264.883 | 389.513 | 532.362 | 6.3 | 2187 |
| spread-100k | 100000 | 0 | 345.189 | 402.949 | 601.872 | 644.709 | 2.9 | 2743 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

