# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 73.273 | 129.639 | 198.391 | 271.968 | 13.6 | 1531 |
| mobs-25k | 25000 | 0 | 565.213 | 633.275 | 749.475 | 1015.205 | 1.8 | 1901 |
| mixed-150k | 150000 | 0 | 579.646 | 804.617 | 1149.974 | 1294.510 | 1.7 | 1893 |
| items-100k-mod | 100000 | 0 | 32.062 | 49.414 | 117.534 | 223.687 | 20.0 | 1475 |
| mixed-150k-mod | 150000 | 0 | 849.136 | 974.272 | 1215.806 | 1496.678 | 1.2 | 2019 |
| mobs-25k-mod | 25000 | 0 | 656.459 | 876.211 | 1085.823 | 1128.392 | 1.5 | 2163 |
| spread-100k-mod | 100000 | 0 | 519.634 | 687.425 | 900.049 | 1151.854 | 1.9 | 2832 |
| spread-100k | 100000 | 0 | 530.282 | 633.178 | 846.556 | 929.392 | 1.9 | 2322 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

