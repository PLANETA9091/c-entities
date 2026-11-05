# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.979 | 81.804 | 147.703 | 197.686 | 15.4 | 2253 |
| mobs-25k | 25000 | 0 | 510.562 | 564.117 | 619.827 | 664.890 | 2.0 | 916 |
| mixed-150k | 150000 | 0 | 672.740 | 793.870 | 1154.062 | 1343.007 | 1.5 | 2165 |
| items-100k-mod | 100000 | 0 | 29.980 | 88.991 | 149.381 | 206.865 | 20.0 | 1576 |
| mixed-150k-mod | 150000 | 0 | 392.337 | 490.859 | 651.720 | 732.463 | 2.5 | 1888 |
| mobs-25k-mod | 25000 | 0 | 352.153 | 583.641 | 697.203 | 798.550 | 2.8 | 1909 |
| spread-100k-mod | 100000 | 0 | 350.662 | 465.351 | 594.360 | 809.132 | 2.9 | 3100 |
| spread-100k | 100000 | 0 | 437.992 | 497.316 | 707.272 | 774.976 | 2.3 | 2054 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

