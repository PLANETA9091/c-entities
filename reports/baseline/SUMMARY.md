# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 68.429 | 83.838 | 149.125 | 202.680 | 14.6 | 1514 |
| mobs-25k | 25000 | 0 | 491.773 | 548.431 | 597.897 | 681.335 | 2.0 | 1274 |
| mixed-150k | 150000 | 0 | 677.712 | 882.837 | 1216.406 | 1372.273 | 1.5 | 3268 |
| items-100k-mod | 100000 | 0 | 29.468 | 82.093 | 144.273 | 174.796 | 20.0 | 1121 |
| mixed-150k-mod | 150000 | 0 | 557.319 | 711.029 | 1008.653 | 1133.656 | 1.8 | 1875 |
| mobs-25k-mod | 25000 | 0 | 284.122 | 479.888 | 724.744 | 808.367 | 3.5 | 1413 |
| spread-100k-mod | 100000 | 0 | 200.891 | 345.606 | 562.102 | 755.438 | 5.0 | 2026 |
| spread-100k | 100000 | 0 | 520.577 | 594.907 | 830.677 | 909.333 | 1.9 | 2068 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

