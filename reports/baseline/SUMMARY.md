# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 79.638 | 96.407 | 174.935 | 223.647 | 12.6 | 896 |
| mobs-25k | 25000 | 0 | 542.908 | 590.937 | 648.510 | 679.428 | 1.8 | 1495 |
| mixed-150k | 150000 | 0 | 649.674 | 786.127 | 1134.792 | 1229.009 | 1.5 | 2450 |
| items-100k-mod | 100000 | 0 | 31.403 | 69.667 | 144.554 | 193.752 | 20.0 | 1585 |
| mixed-150k-mod | 150000 | 0 | 259.605 | 363.814 | 509.639 | 612.595 | 3.9 | 1896 |
| mobs-25k-mod | 25000 | 0 | 292.450 | 520.213 | 677.398 | 744.186 | 3.4 | 1378 |
| spread-100k-mod | 100000 | 0 | 302.978 | 474.469 | 610.388 | 814.158 | 3.3 | 1977 |
| spread-100k | 100000 | 0 | 518.224 | 581.233 | 824.861 | 876.056 | 1.9 | 2564 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

