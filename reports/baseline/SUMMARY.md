# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.830 | 85.387 | 152.410 | 210.443 | 15.0 | 1582 |
| mobs-25k | 25000 | 0 | 577.108 | 660.101 | 714.505 | 783.603 | 1.7 | 1162 |
| mixed-150k | 150000 | 0 | 724.608 | 900.431 | 1374.635 | 1866.433 | 1.4 | 2283 |
| items-100k-mod | 100000 | 0 | 35.102 | 90.333 | 149.325 | 193.787 | 20.0 | 1305 |
| mixed-150k-mod | 150000 | 0 | 271.914 | 385.438 | 525.346 | 615.911 | 3.7 | 1730 |
| mobs-25k-mod | 25000 | 0 | 290.874 | 565.134 | 679.033 | 760.062 | 3.4 | 1676 |
| spread-100k-mod | 100000 | 0 | 315.523 | 480.660 | 589.152 | 770.531 | 3.2 | 1978 |
| spread-100k | 100000 | 0 | 638.473 | 731.008 | 1018.324 | 1135.022 | 1.6 | 2737 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

