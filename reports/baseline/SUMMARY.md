# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 65.087 | 79.720 | 148.388 | 201.148 | 15.4 | 1638 |
| mobs-25k | 25000 | 0 | 530.629 | 585.017 | 634.686 | 670.734 | 1.9 | 1086 |
| mixed-150k | 150000 | 0 | 672.523 | 837.861 | 1204.731 | 1326.416 | 1.5 | 2624 |
| items-100k-mod | 100000 | 0 | 36.716 | 85.697 | 158.523 | 240.056 | 20.0 | 1335 |
| mixed-150k-mod | 150000 | 0 | 393.170 | 498.525 | 758.732 | 858.968 | 2.5 | 1777 |
| mobs-25k-mod | 25000 | 0 | 196.608 | 378.378 | 635.766 | 720.562 | 5.1 | 1439 |
| spread-100k-mod | 100000 | 0 | 208.146 | 359.326 | 589.204 | 729.283 | 4.8 | 2172 |
| spread-100k | 100000 | 0 | 577.238 | 649.820 | 952.399 | 1032.766 | 1.7 | 2059 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

