# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.226 | 86.210 | 155.738 | 191.512 | 14.7 | 1157 |
| mobs-25k | 25000 | 0 | 732.968 | 820.947 | 888.882 | 933.280 | 1.4 | 1503 |
| mixed-150k | 150000 | 0 | 654.558 | 806.296 | 1099.202 | 1378.583 | 1.5 | 3089 |
| items-100k-mod | 100000 | 0 | 33.979 | 90.046 | 141.061 | 203.796 | 20.0 | 1386 |
| mixed-150k-mod | 150000 | 0 | 295.769 | 417.985 | 527.270 | 681.584 | 3.4 | 1704 |
| mobs-25k-mod | 25000 | 0 | 258.635 | 385.781 | 518.943 | 565.425 | 3.9 | 1658 |
| spread-100k-mod | 100000 | 0 | 287.697 | 444.895 | 542.739 | 674.630 | 3.5 | 1863 |
| spread-100k | 100000 | 0 | 450.971 | 519.601 | 734.097 | 821.675 | 2.2 | 2106 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

