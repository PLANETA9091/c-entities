# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 77.010 | 133.458 | 199.217 | 266.165 | 13.0 | 1433 |
| mobs-25k | 25000 | 0 | 334.276 | 411.373 | 495.142 | 644.002 | 3.0 | 1537 |
| mixed-150k | 150000 | 0 | 359.165 | 494.657 | 663.757 | 932.710 | 2.8 | 2251 |
| items-100k-mod | 100000 | 0 | 32.204 | 58.103 | 143.051 | 250.602 | 20.0 | 1818 |
| mixed-150k-mod | 150000 | 0 | 148.289 | 241.356 | 349.242 | 450.275 | 6.7 | 2281 |
| mobs-25k-mod | 25000 | 0 | 1996.735 | 2173.743 | 2324.012 | 2479.445 | 0.5 | 2102 |
| mobs-25k-native-mod | 25000 | 0 | 21.622 | 43.567 | 117.268 | 141.630 | 20.0 | 1520 |
| mobs-25k-native | 25000 | 0 | 390.590 | 461.102 | 554.921 | 684.238 | 2.6 | 1781 |
| spread-100k-mod | 100000 | 0 | 152.257 | 220.354 | 335.892 | 447.187 | 6.6 | 2944 |
| spread-100k | 100000 | 0 | 414.249 | 507.871 | 625.277 | 864.080 | 2.4 | 1726 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

