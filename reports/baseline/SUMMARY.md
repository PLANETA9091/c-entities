# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 88.826 | 106.924 | 149.542 | 177.936 | 11.3 | 1866 |
| mobs-25k | 25000 | 0 | 528.719 | 579.677 | 624.433 | 664.008 | 1.9 | 1348 |
| mixed-150k | 150000 | 0 | 783.046 | 908.069 | 981.911 | 1127.810 | 1.3 | 2203 |
| items-100k-mod | 100000 | 0 | 63.230 | 132.842 | 164.793 | 217.745 | 15.8 | 1124 |
| mixed-150k-mod | 150000 | 0 | 620.780 | 720.343 | 763.816 | 847.621 | 1.6 | 2858 |
| mobs-25k-mod | 25000 | 0 | 347.300 | 409.293 | 473.006 | 538.185 | 2.9 | 1203 |
| spread-100k-mod | 100000 | 0 | 367.770 | 474.472 | 535.700 | 651.768 | 2.7 | 2256 |
| spread-100k | 100000 | 0 | 433.333 | 503.666 | 587.660 | 662.337 | 2.3 | 2487 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

