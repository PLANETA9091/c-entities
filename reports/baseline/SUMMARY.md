# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.137 | 81.545 | 154.595 | 208.490 | 15.4 | 1194 |
| mobs-25k | 25000 | 0 | 372.572 | 412.259 | 454.272 | 511.106 | 2.7 | 1272 |
| mixed-150k | 150000 | 0 | 606.778 | 790.338 | 1390.589 | 1838.400 | 1.6 | 3320 |
| items-100k-mod | 100000 | 0 | 14.382 | 17.339 | 104.157 | 135.510 | 20.0 | 1379 |
| mixed-150k-mod | 150000 | 0 | 332.171 | 394.320 | 457.735 | 600.969 | 3.0 | 2326 |
| mobs-25k-mod | 25000 | 0 | 192.755 | 279.301 | 355.828 | 413.784 | 5.2 | 1336 |
| spread-100k-mod | 100000 | 0 | 134.931 | 199.546 | 271.899 | 339.733 | 7.4 | 2188 |
| spread-100k | 100000 | 0 | 358.782 | 433.081 | 638.796 | 679.126 | 2.8 | 2265 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

