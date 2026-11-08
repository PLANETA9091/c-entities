# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.165 | 132.327 | 190.250 | 268.715 | 14.1 | 878 |
| mobs-25k | 25000 | 0 | 555.872 | 628.392 | 722.020 | 776.284 | 1.8 | 929 |
| mixed-150k | 150000 | 0 | 389.586 | 543.580 | 684.512 | 947.906 | 2.6 | 2443 |
| items-100k-mod | 100000 | 0 | 30.173 | 48.914 | 112.884 | 185.493 | 20.0 | 1585 |
| mixed-150k-mod | 150000 | 0 | 533.296 | 649.322 | 830.405 | 940.032 | 1.9 | 2477 |
| mobs-25k-mod | 25000 | 0 | 674.016 | 881.516 | 1076.340 | 1130.826 | 1.5 | 2342 |
| spread-100k-mod | 100000 | 0 | 468.385 | 633.778 | 802.158 | 974.892 | 2.1 | 2672 |
| spread-100k | 100000 | 0 | 437.235 | 543.658 | 765.509 | 860.654 | 2.3 | 2821 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

