# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.985 | 89.284 | 159.061 | 197.634 | 13.9 | 927 |
| mobs-25k | 25000 | 0 | 511.289 | 602.572 | 679.347 | 730.629 | 2.0 | 1214 |
| mixed-150k | 150000 | 0 | 713.040 | 840.341 | 1133.731 | 1249.484 | 1.4 | 2457 |
| items-100k-mod | 100000 | 0 | 33.851 | 98.405 | 184.744 | 325.505 | 20.0 | 1528 |
| mixed-150k-mod | 150000 | 0 | 390.629 | 486.790 | 622.869 | 788.841 | 2.6 | 1914 |
| mobs-25k-mod | 25000 | 0 | 342.132 | 552.055 | 677.796 | 753.601 | 2.9 | 1766 |
| spread-100k-mod | 100000 | 0 | 350.746 | 462.901 | 588.316 | 751.071 | 2.9 | 2176 |
| spread-100k | 100000 | 0 | 386.492 | 453.287 | 668.099 | 732.886 | 2.6 | 2001 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

