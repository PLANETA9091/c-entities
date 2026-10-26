# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.030 | 76.508 | 146.226 | 224.065 | 15.9 | 1946 |
| mobs-25k | 25000 | 0 | 260.281 | 300.618 | 340.842 | 353.592 | 3.8 | 1710 |
| mixed-150k | 150000 | 0 | 378.375 | 451.239 | 631.125 | 729.741 | 2.6 | 2410 |
| items-100k-mod | 100000 | 0 | 12.755 | 16.446 | 94.962 | 142.121 | 20.0 | 1344 |
| mixed-150k-mod | 150000 | 0 | 254.705 | 306.122 | 350.928 | 870.837 | 3.9 | 1996 |
| mobs-25k-mod | 25000 | 0 | 174.544 | 244.184 | 313.368 | 383.440 | 5.7 | 1520 |
| spread-100k-mod | 100000 | 0 | 116.186 | 172.567 | 226.063 | 282.378 | 8.6 | 1957 |
| spread-100k | 100000 | 0 | 275.053 | 342.998 | 494.250 | 602.686 | 3.6 | 2218 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

