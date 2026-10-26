# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.065 | 80.942 | 144.607 | 208.178 | 15.6 | 815 |
| mobs-25k | 25000 | 0 | 365.889 | 405.638 | 439.427 | 481.868 | 2.7 | 1772 |
| mixed-150k | 150000 | 0 | 390.797 | 474.380 | 700.177 | 784.719 | 2.6 | 2596 |
| items-100k-mod | 100000 | 0 | 13.437 | 17.683 | 79.592 | 110.691 | 20.0 | 1187 |
| mixed-150k-mod | 150000 | 0 | 265.174 | 309.144 | 365.843 | 649.938 | 3.8 | 2734 |
| mobs-25k-mod | 25000 | 0 | 161.121 | 250.821 | 319.289 | 342.093 | 6.2 | 1896 |
| spread-100k-mod | 100000 | 0 | 150.812 | 229.444 | 298.020 | 426.213 | 6.6 | 1989 |
| spread-100k | 100000 | 0 | 393.781 | 449.898 | 646.193 | 691.105 | 2.5 | 2049 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

