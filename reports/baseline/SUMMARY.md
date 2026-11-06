# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.855 | 88.841 | 165.228 | 220.128 | 14.1 | 841 |
| mobs-25k | 25000 | 0 | 521.636 | 592.717 | 664.258 | 717.121 | 1.9 | 1850 |
| mixed-150k | 150000 | 0 | 649.062 | 786.935 | 1133.493 | 1222.800 | 1.5 | 3087 |
| items-100k-mod | 100000 | 0 | 32.815 | 95.152 | 167.780 | 191.222 | 20.0 | 1430 |
| mixed-150k-mod | 150000 | 0 | 411.392 | 508.944 | 668.867 | 780.471 | 2.4 | 1908 |
| mobs-25k-mod | 25000 | 0 | 350.345 | 556.116 | 717.856 | 804.266 | 2.9 | 1996 |
| spread-100k-mod | 100000 | 0 | 313.349 | 442.679 | 567.211 | 752.080 | 3.2 | 2682 |
| spread-100k | 100000 | 0 | 412.357 | 469.011 | 662.993 | 705.190 | 2.4 | 2033 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

