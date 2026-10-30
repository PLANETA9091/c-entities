# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.292 | 78.663 | 139.669 | 174.525 | 15.8 | 1652 |
| mobs-25k | 25000 | 0 | 526.137 | 578.369 | 626.529 | 646.983 | 1.9 | 1758 |
| mixed-150k | 150000 | 0 | 684.235 | 862.961 | 1292.501 | 1483.687 | 1.5 | 3017 |
| items-100k-mod | 100000 | 0 | 29.112 | 77.695 | 136.790 | 233.454 | 20.0 | 1146 |
| mixed-150k-mod | 150000 | 0 | 319.793 | 444.557 | 645.593 | 728.430 | 3.1 | 1853 |
| mobs-25k-mod | 25000 | 0 | 277.722 | 542.690 | 709.441 | 761.132 | 3.6 | 1402 |
| spread-100k-mod | 100000 | 0 | 361.825 | 562.310 | 745.786 | 1018.154 | 2.8 | 1911 |
| spread-100k | 100000 | 0 | 713.155 | 828.141 | 1141.968 | 1283.032 | 1.4 | 2124 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

