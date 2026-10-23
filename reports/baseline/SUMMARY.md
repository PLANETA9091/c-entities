# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 37.595 | 45.789 | 97.714 | 129.916 | 20.0 | 1177 |
| mobs-25k | 25000 | 0 | 366.312 | 403.189 | 446.032 | 513.952 | 2.7 | 1828 |
| mixed-150k | 150000 | 0 | 458.267 | 538.017 | 782.751 | 930.693 | 2.2 | 2197 |
| items-100k-mod | 100000 | 0 | 20.116 | 33.059 | 81.410 | 109.966 | 20.0 | 1073 |
| mixed-150k-mod | 150000 | 0 | 396.749 | 452.881 | 498.973 | 559.652 | 2.5 | 1752 |
| mobs-25k-mod | 25000 | 0 | 192.965 | 236.525 | 272.259 | 319.863 | 5.2 | 1745 |
| spread-100k-mod | 100000 | 0 | 241.870 | 309.030 | 406.971 | 604.184 | 4.1 | 1984 |
| spread-100k | 100000 | 0 | 303.591 | 373.907 | 547.762 | 621.677 | 3.3 | 2644 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

