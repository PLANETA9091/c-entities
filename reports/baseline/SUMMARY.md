# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 76.066 | 96.427 | 181.222 | 217.512 | 13.1 | 1539 |
| mobs-25k | 25000 | 0 | 494.219 | 548.832 | 599.105 | 634.895 | 2.0 | 1101 |
| mixed-150k | 150000 | 0 | 452.013 | 580.222 | 791.620 | 1222.783 | 2.2 | 2543 |
| items-100k-mod | 100000 | 0 | 37.250 | 88.338 | 165.344 | 275.433 | 20.0 | 1544 |
| mixed-150k-mod | 150000 | 0 | 227.939 | 318.444 | 421.077 | 624.704 | 4.4 | 2317 |
| mobs-25k-mod | 25000 | 0 | 242.487 | 347.017 | 491.826 | 543.258 | 4.1 | 1262 |
| spread-100k-mod | 100000 | 0 | 260.838 | 419.230 | 520.212 | 682.216 | 3.8 | 2112 |
| spread-100k | 100000 | 0 | 581.231 | 657.846 | 863.446 | 928.372 | 1.7 | 2169 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

