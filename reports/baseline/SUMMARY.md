# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.313 | 89.765 | 162.784 | 208.853 | 14.0 | 1660 |
| mobs-25k | 25000 | 0 | 354.356 | 405.482 | 445.821 | 485.134 | 2.8 | 1857 |
| mixed-150k | 150000 | 0 | 439.517 | 525.509 | 729.946 | 804.105 | 2.3 | 2711 |
| items-100k-mod | 100000 | 0 | 32.456 | 83.420 | 168.523 | 202.477 | 20.0 | 1582 |
| mixed-150k-mod | 150000 | 0 | 284.937 | 371.546 | 474.478 | 616.173 | 3.5 | 2073 |
| mobs-25k-mod | 25000 | 0 | 232.515 | 397.533 | 523.343 | 632.998 | 4.3 | 1942 |
| spread-100k-mod | 100000 | 0 | 393.716 | 508.216 | 636.787 | 892.601 | 2.5 | 2481 |
| spread-100k | 100000 | 0 | 437.721 | 509.957 | 722.694 | 801.938 | 2.3 | 2212 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

