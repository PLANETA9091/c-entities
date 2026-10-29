# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 58.927 | 73.602 | 135.651 | 212.009 | 17.0 | 1218 |
| mobs-25k | 25000 | 0 | 509.160 | 572.913 | 627.764 | 669.653 | 2.0 | 1460 |
| mixed-150k | 150000 | 0 | 693.407 | 859.903 | 1253.817 | 1378.952 | 1.4 | 2756 |
| items-100k-mod | 100000 | 0 | 24.370 | 60.248 | 132.308 | 229.590 | 20.0 | 1189 |
| mixed-150k-mod | 150000 | 0 | 412.214 | 513.159 | 684.549 | 741.071 | 2.4 | 1842 |
| mobs-25k-mod | 25000 | 0 | 190.656 | 394.184 | 712.536 | 872.383 | 5.2 | 1444 |
| spread-100k-mod | 100000 | 0 | 150.764 | 232.782 | 333.768 | 411.507 | 6.6 | 2828 |
| spread-100k | 100000 | 0 | 322.117 | 381.246 | 548.675 | 648.704 | 3.1 | 2767 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

