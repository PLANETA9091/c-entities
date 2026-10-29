# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.259 | 83.272 | 144.893 | 199.048 | 15.3 | 2028 |
| mobs-25k | 25000 | 0 | 326.013 | 370.126 | 408.505 | 472.924 | 3.1 | 1844 |
| mixed-150k | 150000 | 0 | 637.792 | 767.799 | 995.425 | 1075.759 | 1.6 | 1742 |
| items-100k-mod | 100000 | 0 | 24.680 | 59.432 | 121.336 | 198.620 | 20.0 | 1395 |
| mixed-150k-mod | 150000 | 0 | 385.520 | 463.086 | 657.448 | 694.581 | 2.6 | 1769 |
| mobs-25k-mod | 25000 | 0 | 143.734 | 276.369 | 526.382 | 652.345 | 7.0 | 1588 |
| spread-100k-mod | 100000 | 0 | 203.184 | 349.293 | 510.569 | 770.950 | 4.9 | 1927 |
| spread-100k | 100000 | 0 | 528.997 | 606.227 | 843.910 | 934.715 | 1.9 | 2092 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

