# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 74.778 | 92.734 | 175.572 | 217.983 | 13.4 | 1736 |
| mobs-25k | 25000 | 0 | 265.533 | 304.541 | 337.670 | 428.085 | 3.8 | 924 |
| mixed-150k | 150000 | 0 | 521.469 | 644.938 | 947.184 | 1062.002 | 1.9 | 3076 |
| items-100k-mod | 100000 | 0 | 17.531 | 25.490 | 101.543 | 134.032 | 20.0 | 1281 |
| mixed-150k-mod | 150000 | 0 | 311.428 | 409.702 | 583.154 | 630.450 | 3.2 | 2327 |
| mobs-25k-mod | 25000 | 0 | 174.463 | 298.726 | 477.986 | 552.652 | 5.7 | 1359 |
| spread-100k-mod | 100000 | 0 | 154.569 | 283.106 | 416.321 | 485.255 | 6.5 | 2402 |
| spread-100k | 100000 | 0 | 394.602 | 449.776 | 637.445 | 693.105 | 2.5 | 2070 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

