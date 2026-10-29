# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.185 | 92.085 | 159.622 | 192.803 | 14.0 | 1803 |
| mobs-25k | 25000 | 0 | 349.074 | 401.046 | 463.705 | 731.628 | 2.9 | 1193 |
| mixed-150k | 150000 | 0 | 429.204 | 524.518 | 717.573 | 817.355 | 2.3 | 2436 |
| items-100k-mod | 100000 | 0 | 34.735 | 93.500 | 166.889 | 208.557 | 20.0 | 1148 |
| mixed-150k-mod | 150000 | 0 | 243.951 | 352.005 | 473.768 | 646.516 | 4.1 | 2095 |
| mobs-25k-mod | 25000 | 0 | 220.782 | 336.330 | 462.413 | 514.311 | 4.5 | 1607 |
| spread-100k-mod | 100000 | 0 | 257.688 | 406.910 | 514.017 | 674.294 | 3.9 | 2821 |
| spread-100k | 100000 | 0 | 402.522 | 465.826 | 652.331 | 697.910 | 2.5 | 1967 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

