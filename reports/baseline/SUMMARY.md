# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 51.993 | 65.658 | 118.737 | 164.058 | 19.2 | 1921 |
| mobs-25k | 25000 | 0 | 526.259 | 594.745 | 659.045 | 697.631 | 1.9 | 935 |
| mixed-150k | 150000 | 0 | 855.455 | 1012.113 | 1347.900 | 1468.759 | 1.2 | 2054 |
| items-100k-mod | 100000 | 0 | 22.829 | 57.212 | 121.295 | 212.799 | 20.0 | 1707 |
| mixed-150k-mod | 150000 | 0 | 345.527 | 447.648 | 709.118 | 758.915 | 2.9 | 1694 |
| mobs-25k-mod | 25000 | 0 | 182.808 | 426.996 | 733.979 | 889.002 | 5.5 | 2294 |
| spread-100k-mod | 100000 | 0 | 189.306 | 345.747 | 530.689 | 649.043 | 5.3 | 2256 |
| spread-100k | 100000 | 0 | 549.771 | 622.045 | 890.837 | 981.586 | 1.8 | 2146 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

