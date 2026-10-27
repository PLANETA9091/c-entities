# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.311 | 81.800 | 142.765 | 226.266 | 15.3 | 1454 |
| mobs-25k | 25000 | 0 | 327.873 | 377.013 | 423.876 | 432.113 | 3.0 | 1630 |
| mixed-150k | 150000 | 0 | 550.206 | 656.779 | 935.833 | 1117.242 | 1.8 | 2045 |
| items-100k-mod | 100000 | 0 | 11.265 | 15.842 | 76.251 | 103.896 | 20.0 | 1291 |
| mixed-150k-mod | 150000 | 0 | 302.577 | 423.426 | 581.020 | 635.762 | 3.3 | 2726 |
| mobs-25k-mod | 25000 | 0 | 165.964 | 279.896 | 430.439 | 503.012 | 6.0 | 1681 |
| spread-100k-mod | 100000 | 0 | 166.018 | 291.077 | 441.910 | 619.956 | 6.0 | 2035 |
| spread-100k | 100000 | 0 | 396.260 | 459.015 | 664.824 | 795.014 | 2.5 | 2241 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

