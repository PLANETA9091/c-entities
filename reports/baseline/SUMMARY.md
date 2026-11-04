# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.501 | 80.927 | 147.629 | 196.825 | 15.7 | 2019 |
| mobs-25k | 25000 | 0 | 521.355 | 577.362 | 621.491 | 666.680 | 1.9 | 1644 |
| mixed-150k | 150000 | 0 | 639.664 | 769.648 | 975.797 | 1128.935 | 1.6 | 2975 |
| items-100k-mod | 100000 | 0 | 28.606 | 84.325 | 162.151 | 188.433 | 20.0 | 1659 |
| mixed-150k-mod | 150000 | 0 | 318.393 | 420.768 | 545.565 | 647.088 | 3.1 | 2574 |
| mobs-25k-mod | 25000 | 0 | 345.109 | 551.189 | 708.994 | 771.046 | 2.9 | 1348 |
| spread-100k-mod | 100000 | 0 | 433.738 | 573.626 | 716.277 | 913.209 | 2.3 | 2309 |
| spread-100k | 100000 | 0 | 503.510 | 579.931 | 794.452 | 874.253 | 2.0 | 2186 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

