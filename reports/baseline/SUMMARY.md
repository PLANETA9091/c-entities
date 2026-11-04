# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.403 | 84.304 | 149.618 | 199.906 | 15.1 | 2039 |
| mobs-25k | 25000 | 0 | 499.721 | 548.092 | 598.201 | 628.246 | 2.0 | 1363 |
| mixed-150k | 150000 | 0 | 680.023 | 828.176 | 1174.083 | 1407.271 | 1.5 | 1770 |
| items-100k-mod | 100000 | 0 | 31.652 | 92.817 | 179.966 | 223.148 | 20.0 | 1618 |
| mixed-150k-mod | 150000 | 0 | 469.864 | 567.923 | 738.892 | 852.623 | 2.1 | 1751 |
| mobs-25k-mod | 25000 | 0 | 266.541 | 422.213 | 552.802 | 592.851 | 3.8 | 1653 |
| spread-100k-mod | 100000 | 0 | 330.667 | 428.047 | 533.110 | 622.470 | 3.0 | 2754 |
| spread-100k | 100000 | 0 | 457.324 | 524.107 | 724.929 | 753.553 | 2.2 | 2163 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

