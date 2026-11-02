# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 60.859 | 74.959 | 142.325 | 222.980 | 16.4 | 899 |
| mobs-25k | 25000 | 0 | 393.931 | 442.835 | 488.684 | 505.848 | 2.5 | 1252 |
| mixed-150k | 150000 | 0 | 665.984 | 810.633 | 1129.152 | 1254.221 | 1.5 | 2110 |
| items-100k-mod | 100000 | 0 | 20.753 | 46.927 | 122.816 | 183.262 | 20.0 | 1469 |
| mixed-150k-mod | 150000 | 0 | 263.232 | 381.175 | 536.937 | 651.528 | 3.8 | 1905 |
| mobs-25k-mod | 25000 | 0 | 231.816 | 395.232 | 527.052 | 671.673 | 4.3 | 1301 |
| spread-100k-mod | 100000 | 0 | 260.499 | 420.130 | 530.829 | 699.754 | 3.8 | 2125 |
| spread-100k | 100000 | 0 | 581.658 | 657.970 | 868.552 | 951.780 | 1.7 | 1990 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

