# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.388 | 93.301 | 162.825 | 224.039 | 14.0 | 1321 |
| mobs-25k | 25000 | 0 | 510.522 | 569.755 | 631.717 | 765.353 | 2.0 | 1518 |
| mixed-150k | 150000 | 0 | 451.089 | 550.746 | 759.646 | 862.712 | 2.2 | 2973 |
| items-100k-mod | 100000 | 0 | 36.646 | 96.696 | 171.800 | 252.480 | 20.0 | 1270 |
| mixed-150k-mod | 150000 | 0 | 228.635 | 328.617 | 455.086 | 533.804 | 4.4 | 1673 |
| mobs-25k-mod | 25000 | 0 | 278.180 | 447.584 | 565.987 | 668.609 | 3.6 | 1913 |
| spread-100k-mod | 100000 | 0 | 246.056 | 353.366 | 449.479 | 542.145 | 4.1 | 2243 |
| spread-100k | 100000 | 0 | 398.851 | 468.632 | 680.517 | 754.110 | 2.5 | 1862 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

