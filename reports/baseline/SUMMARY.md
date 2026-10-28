# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.258 | 77.485 | 142.255 | 187.116 | 16.1 | 737 |
| mobs-25k | 25000 | 0 | 503.500 | 552.125 | 608.198 | 737.422 | 2.0 | 1802 |
| mixed-150k | 150000 | 0 | 836.341 | 1035.887 | 1657.378 | 2205.087 | 1.2 | 2379 |
| items-100k-mod | 100000 | 0 | 25.823 | 62.066 | 131.106 | 174.885 | 20.0 | 1292 |
| mixed-150k-mod | 150000 | 0 | 523.032 | 790.846 | 1044.339 | 1388.090 | 1.9 | 1832 |
| mobs-25k-mod | 25000 | 0 | 223.370 | 377.725 | 602.162 | 681.847 | 4.5 | 1880 |
| spread-100k-mod | 100000 | 0 | 176.525 | 291.694 | 425.353 | 503.721 | 5.7 | 1817 |
| spread-100k | 100000 | 0 | 377.135 | 443.534 | 638.009 | 718.063 | 2.7 | 3007 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

