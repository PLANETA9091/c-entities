# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.295 | 77.369 | 140.347 | 188.275 | 15.8 | 836 |
| mobs-25k | 25000 | 0 | 379.292 | 423.368 | 461.206 | 478.895 | 2.6 | 1408 |
| mixed-150k | 150000 | 0 | 371.300 | 446.792 | 610.802 | 746.372 | 2.7 | 2126 |
| items-100k-mod | 100000 | 0 | 15.163 | 19.224 | 76.868 | 122.915 | 20.0 | 1452 |
| mixed-150k-mod | 150000 | 0 | 233.971 | 286.404 | 334.588 | 376.850 | 4.3 | 2209 |
| mobs-25k-mod | 25000 | 0 | 193.222 | 328.644 | 422.826 | 465.646 | 5.2 | 1915 |
| spread-100k-mod | 100000 | 0 | 159.771 | 249.567 | 335.219 | 474.303 | 6.3 | 1812 |
| spread-100k | 100000 | 0 | 406.860 | 470.371 | 665.966 | 711.895 | 2.5 | 2078 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

