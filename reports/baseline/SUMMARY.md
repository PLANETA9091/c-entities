# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.164 | 78.659 | 144.370 | 208.638 | 15.8 | 2290 |
| mobs-25k | 25000 | 0 | 393.602 | 444.447 | 489.449 | 527.746 | 2.5 | 923 |
| mixed-150k | 150000 | 0 | 380.463 | 459.994 | 657.360 | 740.180 | 2.6 | 2362 |
| items-100k-mod | 100000 | 0 | 10.390 | 14.650 | 72.383 | 103.715 | 20.0 | 1327 |
| mixed-150k-mod | 150000 | 0 | 237.590 | 317.266 | 411.407 | 477.280 | 4.2 | 2590 |
| mobs-25k-mod | 25000 | 0 | 186.666 | 306.509 | 484.325 | 578.575 | 5.4 | 1837 |
| spread-100k-mod | 100000 | 0 | 175.473 | 321.053 | 486.115 | 666.763 | 5.7 | 2353 |
| spread-100k | 100000 | 0 | 434.573 | 508.758 | 722.502 | 947.355 | 2.3 | 2053 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

