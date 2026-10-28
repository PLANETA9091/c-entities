# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.984 | 84.756 | 151.661 | 196.695 | 15.4 | 885 |
| mobs-25k | 25000 | 0 | 542.962 | 611.356 | 663.241 | 680.024 | 1.8 | 960 |
| mixed-150k | 150000 | 0 | 627.407 | 770.395 | 1011.120 | 1130.166 | 1.6 | 3440 |
| items-100k-mod | 100000 | 0 | 34.153 | 89.742 | 160.816 | 204.341 | 20.0 | 1181 |
| mixed-150k-mod | 150000 | 0 | 464.013 | 570.778 | 881.233 | 1008.904 | 2.2 | 2019 |
| mobs-25k-mod | 25000 | 0 | 262.563 | 454.197 | 667.320 | 718.770 | 3.8 | 1241 |
| spread-100k-mod | 100000 | 0 | 174.199 | 274.615 | 395.698 | 434.300 | 5.7 | 2748 |
| spread-100k | 100000 | 0 | 432.005 | 502.926 | 727.688 | 783.659 | 2.3 | 2269 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

