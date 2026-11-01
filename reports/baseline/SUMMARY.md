# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.008 | 85.633 | 154.924 | 200.003 | 14.3 | 1201 |
| mobs-25k | 25000 | 0 | 319.522 | 371.073 | 423.246 | 467.527 | 3.1 | 980 |
| mixed-150k | 150000 | 0 | 729.989 | 861.108 | 1118.309 | 1251.957 | 1.4 | 1895 |
| items-100k-mod | 100000 | 0 | 34.665 | 87.898 | 161.376 | 207.240 | 20.0 | 1488 |
| mixed-150k-mod | 150000 | 0 | 257.328 | 353.366 | 472.280 | 570.260 | 3.9 | 1840 |
| mobs-25k-mod | 25000 | 0 | 216.807 | 317.503 | 477.398 | 532.630 | 4.6 | 1213 |
| spread-100k-mod | 100000 | 0 | 289.246 | 444.423 | 562.475 | 709.852 | 3.5 | 1952 |
| spread-100k | 100000 | 0 | 613.531 | 684.877 | 952.802 | 1038.851 | 1.6 | 2142 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

