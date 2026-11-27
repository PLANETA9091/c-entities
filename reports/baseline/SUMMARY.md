# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 70.813 | 126.059 | 191.561 | 264.993 | 14.1 | 2797 |
| mobs-25k | 25000 | 0 | 445.812 | 524.850 | 692.367 | 794.693 | 2.2 | 1218 |
| mixed-150k | 150000 | 0 | 569.497 | 756.278 | 966.770 | 1054.972 | 1.8 | 1929 |
| items-100k-mod | 100000 | 0 | 15.871 | 32.716 | 106.177 | 183.881 | 20.0 | 1286 |
| mixed-150k-mod | 150000 | 0 | 50.551 | 90.341 | 193.698 | 273.738 | 19.8 | 1840 |
| mobs-25k-mod | 25000 | 0 | 3124.613 | 3430.535 | 3698.409 | 4001.723 | 0.3 | 1729 |
| mobs-25k-native-mod | 25000 | 0 | 22.566 | 39.427 | 118.177 | 175.901 | 20.0 | 1397 |
| mobs-25k-native | 25000 | 0 | 746.809 | 830.879 | 900.866 | 945.147 | 1.3 | 1384 |
| spread-100k-mod | 100000 | 0 | 46.968 | 71.550 | 197.154 | 348.417 | 20.0 | 2295 |
| spread-100k | 100000 | 0 | 424.058 | 518.527 | 610.292 | 706.964 | 2.4 | 2006 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

