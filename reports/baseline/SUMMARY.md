# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 74.210 | 92.783 | 174.445 | 218.151 | 13.5 | 959 |
| mobs-25k | 25000 | 0 | 508.692 | 567.522 | 614.932 | 634.827 | 2.0 | 1202 |
| mixed-150k | 150000 | 0 | 540.849 | 663.096 | 868.192 | 1054.274 | 1.8 | 2890 |
| items-100k-mod | 100000 | 0 | 34.407 | 96.023 | 159.172 | 240.920 | 20.0 | 1367 |
| mixed-150k-mod | 150000 | 0 | 362.896 | 454.717 | 661.219 | 710.898 | 2.8 | 1869 |
| mobs-25k-mod | 25000 | 0 | 159.798 | 358.494 | 602.215 | 676.803 | 6.3 | 1343 |
| spread-100k-mod | 100000 | 0 | 151.067 | 250.557 | 378.927 | 487.422 | 6.6 | 2291 |
| spread-100k | 100000 | 0 | 356.098 | 411.013 | 580.868 | 612.461 | 2.8 | 3083 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

