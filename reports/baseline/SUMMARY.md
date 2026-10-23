# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 80.399 | 97.637 | 154.964 | 193.454 | 12.4 | 2102 |
| mobs-25k | 25000 | 0 | 558.900 | 624.037 | 710.262 | 761.353 | 1.8 | 1883 |
| mixed-150k | 150000 | 0 | 869.404 | 1010.195 | 1107.113 | 1214.770 | 1.2 | 3131 |
| items-100k-mod | 100000 | 0 | 14.137 | 30.743 | 43.056 | 81.494 | 20.0 | 1686 |
| mixed-150k-mod | 150000 | 0 | 467.049 | 572.117 | 635.429 | 695.758 | 2.1 | 2105 |
| mobs-25k-mod | 25000 | 0 | 326.284 | 389.758 | 456.045 | 483.254 | 3.1 | 1725 |
| spread-100k-mod | 100000 | 0 | 430.899 | 536.768 | 647.761 | 753.596 | 2.3 | 2527 |
| spread-100k | 100000 | 0 | 467.141 | 555.404 | 671.065 | 882.201 | 2.1 | 3336 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

