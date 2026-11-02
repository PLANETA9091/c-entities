# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.920 | 91.845 | 169.704 | 239.321 | 13.9 | 1326 |
| mobs-25k | 25000 | 0 | 411.291 | 456.355 | 507.996 | 541.977 | 2.4 | 1139 |
| mixed-150k | 150000 | 0 | 685.066 | 807.437 | 1123.135 | 1303.933 | 1.5 | 1955 |
| items-100k-mod | 100000 | 0 | 24.760 | 54.221 | 129.840 | 150.515 | 20.0 | 1606 |
| mixed-150k-mod | 150000 | 0 | 266.148 | 369.130 | 564.711 | 666.237 | 3.8 | 2036 |
| mobs-25k-mod | 25000 | 0 | 244.059 | 392.895 | 548.742 | 612.947 | 4.1 | 1941 |
| spread-100k-mod | 100000 | 0 | 285.323 | 445.398 | 633.888 | 876.829 | 3.5 | 2077 |
| spread-100k | 100000 | 0 | 522.984 | 591.306 | 849.509 | 888.446 | 1.9 | 2166 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

