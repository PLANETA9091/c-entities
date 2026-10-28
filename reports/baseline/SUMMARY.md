# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 55.245 | 67.682 | 125.444 | 185.928 | 18.1 | 1802 |
| mobs-25k | 25000 | 0 | 511.420 | 565.244 | 602.539 | 625.828 | 2.0 | 1754 |
| mixed-150k | 150000 | 0 | 507.602 | 635.920 | 936.162 | 1014.520 | 2.0 | 3010 |
| items-100k-mod | 100000 | 0 | 22.843 | 58.049 | 113.394 | 170.650 | 20.0 | 1243 |
| mixed-150k-mod | 150000 | 0 | 424.467 | 568.181 | 769.259 | 888.874 | 2.4 | 1824 |
| mobs-25k-mod | 25000 | 0 | 234.532 | 402.352 | 605.922 | 733.820 | 4.3 | 1649 |
| spread-100k-mod | 100000 | 0 | 167.758 | 301.585 | 456.242 | 639.317 | 6.0 | 2525 |
| spread-100k | 100000 | 0 | 400.943 | 465.288 | 665.641 | 717.435 | 2.5 | 2275 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

