# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.745 | 89.189 | 155.930 | 205.529 | 13.9 | 826 |
| mobs-25k | 25000 | 0 | 509.873 | 570.213 | 618.707 | 672.207 | 2.0 | 934 |
| mixed-150k | 150000 | 0 | 637.026 | 788.170 | 1069.522 | 1192.908 | 1.6 | 3056 |
| items-100k-mod | 100000 | 0 | 31.199 | 87.150 | 148.007 | 197.992 | 20.0 | 1390 |
| mixed-150k-mod | 150000 | 0 | 659.494 | 790.775 | 1169.630 | 1416.456 | 1.5 | 2298 |
| mobs-25k-mod | 25000 | 0 | 545.983 | 744.009 | 872.129 | 931.192 | 1.8 | 1887 |
| spread-100k-mod | 100000 | 0 | 548.702 | 664.002 | 943.776 | 1207.981 | 1.8 | 2215 |
| spread-100k | 100000 | 0 | 552.934 | 633.398 | 888.891 | 982.709 | 1.8 | 2030 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

