# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.640 | 85.398 | 155.702 | 213.800 | 14.6 | 1250 |
| mobs-25k | 25000 | 0 | 321.331 | 374.520 | 427.858 | 449.731 | 3.1 | 1700 |
| mixed-150k | 150000 | 0 | 632.315 | 779.227 | 1034.150 | 1149.168 | 1.6 | 3067 |
| items-100k-mod | 100000 | 0 | 34.155 | 95.625 | 164.875 | 211.414 | 20.0 | 1403 |
| mixed-150k-mod | 150000 | 0 | 434.815 | 565.541 | 831.918 | 981.828 | 2.3 | 2080 |
| mobs-25k-mod | 25000 | 0 | 183.843 | 292.090 | 497.494 | 567.901 | 5.4 | 1918 |
| spread-100k-mod | 100000 | 0 | 207.918 | 361.115 | 536.774 | 709.213 | 4.8 | 1883 |
| spread-100k | 100000 | 0 | 568.807 | 654.142 | 921.258 | 1104.955 | 1.8 | 2076 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

