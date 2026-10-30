# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.642 | 81.731 | 149.652 | 218.497 | 14.8 | 1132 |
| mobs-25k | 25000 | 0 | 408.169 | 458.776 | 503.952 | 539.314 | 2.4 | 1673 |
| mixed-150k | 150000 | 0 | 921.239 | 1172.659 | 1873.758 | 2130.670 | 1.1 | 3015 |
| items-100k-mod | 100000 | 0 | 34.817 | 93.176 | 165.148 | 218.355 | 20.0 | 1199 |
| mixed-150k-mod | 150000 | 0 | 373.710 | 494.057 | 757.618 | 882.323 | 2.7 | 1860 |
| mobs-25k-mod | 25000 | 0 | 226.527 | 333.178 | 481.974 | 578.492 | 4.4 | 1574 |
| spread-100k-mod | 100000 | 0 | 232.025 | 340.273 | 412.850 | 553.298 | 4.3 | 2521 |
| spread-100k | 100000 | 0 | 375.132 | 435.380 | 621.447 | 702.064 | 2.7 | 1976 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

