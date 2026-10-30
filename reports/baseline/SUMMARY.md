# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.383 | 87.379 | 154.214 | 226.062 | 14.6 | 2097 |
| mobs-25k | 25000 | 0 | 399.784 | 447.188 | 493.160 | 521.731 | 2.5 | 1221 |
| mixed-150k | 150000 | 0 | 628.531 | 739.356 | 950.285 | 1103.411 | 1.6 | 1776 |
| items-100k-mod | 100000 | 0 | 35.365 | 83.756 | 171.757 | 195.621 | 20.0 | 1458 |
| mixed-150k-mod | 150000 | 0 | 248.133 | 350.921 | 509.136 | 670.509 | 4.0 | 1758 |
| mobs-25k-mod | 25000 | 0 | 223.825 | 347.659 | 471.890 | 509.276 | 4.5 | 1195 |
| spread-100k-mod | 100000 | 0 | 272.832 | 426.589 | 516.135 | 641.744 | 3.7 | 1936 |
| spread-100k | 100000 | 0 | 495.588 | 574.383 | 772.723 | 842.267 | 2.0 | 2035 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

