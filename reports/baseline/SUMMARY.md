# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 56.212 | 96.734 | 144.395 | 188.584 | 17.8 | 1577 |
| mobs-25k | 25000 | 0 | 552.464 | 623.815 | 725.546 | 956.021 | 1.8 | 1244 |
| mixed-150k | 150000 | 0 | 590.231 | 889.849 | 1251.824 | 1533.533 | 1.7 | 2224 |
| items-100k-mod | 100000 | 0 | 21.181 | 31.798 | 91.240 | 153.937 | 20.0 | 1834 |
| mixed-150k-mod | 150000 | 0 | 858.572 | 1002.456 | 1247.397 | 1435.631 | 1.2 | 1870 |
| mobs-25k-mod | 25000 | 0 | 631.464 | 940.647 | 1067.686 | 1118.076 | 1.6 | 1561 |
| spread-100k-mod | 100000 | 0 | 610.972 | 753.948 | 945.472 | 1169.952 | 1.6 | 2185 |
| spread-100k | 100000 | 0 | 591.890 | 710.266 | 961.636 | 1030.044 | 1.7 | 2980 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

