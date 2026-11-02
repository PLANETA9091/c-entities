# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.389 | 88.773 | 165.636 | 219.681 | 14.0 | 721 |
| mobs-25k | 25000 | 0 | 403.182 | 449.724 | 486.227 | 516.147 | 2.5 | 1250 |
| mixed-150k | 150000 | 0 | 556.899 | 663.910 | 942.197 | 1124.475 | 1.8 | 1719 |
| items-100k-mod | 100000 | 0 | 36.453 | 90.894 | 162.215 | 186.735 | 20.0 | 1441 |
| mixed-150k-mod | 150000 | 0 | 117.978 | 153.742 | 219.388 | 322.951 | 8.5 | 1438 |
| mobs-25k-mod | 25000 | 0 | 0.240 | 0.288 | 0.369 | 2.888 | 20.0 | 874 |
| spread-100k-mod | 100000 | 0 | 91.042 | 106.962 | 197.899 | 216.865 | 11.0 | 2346 |
| spread-100k | 100000 | 0 | 430.416 | 492.288 | 702.646 | 767.693 | 2.3 | 2122 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

