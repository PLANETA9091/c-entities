# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.481 | 82.398 | 140.357 | 204.104 | 15.8 | 1195 |
| mobs-25k | 25000 | 0 | 494.612 | 552.827 | 601.416 | 672.673 | 2.0 | 1446 |
| mixed-150k | 150000 | 0 | 732.259 | 1081.439 | 1552.417 | 1903.517 | 1.4 | 2630 |
| items-100k-mod | 100000 | 0 | 26.662 | 65.437 | 132.403 | 197.146 | 20.0 | 1545 |
| mixed-150k-mod | 150000 | 0 | 292.150 | 412.281 | 518.744 | 566.882 | 3.4 | 1702 |
| mobs-25k-mod | 25000 | 0 | 272.964 | 510.281 | 653.797 | 795.467 | 3.7 | 1354 |
| spread-100k-mod | 100000 | 0 | 279.888 | 456.158 | 661.755 | 796.675 | 3.6 | 1998 |
| spread-100k | 100000 | 0 | 438.304 | 518.050 | 718.404 | 898.042 | 2.3 | 2166 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

