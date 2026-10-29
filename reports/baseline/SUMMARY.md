# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 63.482 | 89.545 | 143.193 | 217.491 | 15.8 | 1290 |
| mobs-25k | 25000 | 0 | 509.627 | 575.349 | 631.197 | 659.932 | 2.0 | 2313 |
| mixed-150k | 150000 | 0 | 773.238 | 1005.635 | 1475.919 | 1679.461 | 1.3 | 1572 |
| items-100k-mod | 100000 | 0 | 25.883 | 63.524 | 124.716 | 220.573 | 20.0 | 1626 |
| mixed-150k-mod | 150000 | 0 | 400.175 | 505.215 | 785.687 | 890.478 | 2.5 | 1761 |
| mobs-25k-mod | 25000 | 0 | 208.861 | 425.205 | 682.353 | 840.667 | 4.8 | 1308 |
| spread-100k-mod | 100000 | 0 | 174.941 | 338.777 | 552.885 | 812.293 | 5.7 | 2518 |
| spread-100k | 100000 | 0 | 495.732 | 569.785 | 796.473 | 843.500 | 2.0 | 2151 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

