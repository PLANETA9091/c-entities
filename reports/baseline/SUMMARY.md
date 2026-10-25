# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 54.024 | 68.594 | 126.434 | 164.590 | 18.5 | 1434 |
| mobs-25k | 25000 | 0 | 275.722 | 313.032 | 347.092 | 379.005 | 3.6 | 1325 |
| mixed-150k | 150000 | 0 | 373.955 | 451.103 | 625.236 | 692.863 | 2.7 | 2265 |
| items-100k-mod | 100000 | 0 | 16.591 | 19.470 | 93.415 | 110.368 | 20.0 | 1162 |
| mixed-150k-mod | 150000 | 0 | 239.144 | 276.894 | 327.359 | 383.245 | 4.2 | 2095 |
| mobs-25k-mod | 25000 | 0 | 127.386 | 230.755 | 302.289 | 333.039 | 7.9 | 1603 |
| spread-100k-mod | 100000 | 0 | 127.808 | 211.825 | 296.992 | 418.761 | 7.8 | 2680 |
| spread-100k | 100000 | 0 | 363.874 | 423.204 | 617.135 | 644.142 | 2.7 | 2049 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

