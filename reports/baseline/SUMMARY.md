# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.353 | 78.745 | 145.797 | 217.316 | 15.8 | 1106 |
| mobs-25k | 25000 | 0 | 490.732 | 546.817 | 602.453 | 636.715 | 2.0 | 940 |
| mixed-150k | 150000 | 0 | 570.370 | 673.181 | 936.087 | 1096.147 | 1.8 | 3137 |
| items-100k-mod | 100000 | 0 | 26.223 | 70.711 | 138.009 | 178.164 | 20.0 | 1628 |
| mixed-150k-mod | 150000 | 0 | 251.602 | 354.758 | 550.459 | 605.867 | 4.0 | 1792 |
| mobs-25k-mod | 25000 | 0 | 293.257 | 585.193 | 698.778 | 803.981 | 3.4 | 1492 |
| spread-100k-mod | 100000 | 0 | 318.380 | 499.652 | 684.734 | 899.007 | 3.1 | 2174 |
| spread-100k | 100000 | 0 | 632.854 | 713.026 | 981.926 | 1122.229 | 1.6 | 2031 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

