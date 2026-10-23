# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 77.924 | 91.724 | 136.653 | 199.154 | 12.8 | 1827 |
| mobs-25k | 25000 | 0 | 577.174 | 640.409 | 714.920 | 796.789 | 1.7 | 1226 |
| mixed-150k | 150000 | 0 | 670.132 | 765.956 | 840.514 | 1426.984 | 1.5 | 2282 |
| items-100k-mod | 100000 | 0 | 50.343 | 54.110 | 69.143 | 101.658 | 19.9 | 1024 |
| mixed-150k-mod | 150000 | 0 | 574.770 | 644.719 | 729.385 | 1347.614 | 1.7 | 2081 |
| mobs-25k-mod | 25000 | 0 | 349.272 | 401.030 | 461.562 | 551.402 | 2.9 | 1723 |
| spread-100k-mod | 100000 | 0 | 324.706 | 397.103 | 464.872 | 533.332 | 3.1 | 2351 |
| spread-100k | 100000 | 0 | 455.573 | 527.871 | 630.054 | 764.206 | 2.2 | 3196 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

