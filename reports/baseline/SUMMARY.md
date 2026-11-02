# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 76.877 | 96.690 | 185.304 | 249.468 | 13.0 | 2216 |
| mobs-25k | 25000 | 0 | 520.030 | 576.974 | 628.563 | 672.997 | 1.9 | 1988 |
| mixed-150k | 150000 | 0 | 539.611 | 643.470 | 921.221 | 1034.764 | 1.9 | 2497 |
| items-100k-mod | 100000 | 0 | 34.683 | 96.185 | 160.198 | 207.341 | 20.0 | 1239 |
| mixed-150k-mod | 150000 | 0 | 224.755 | 321.180 | 453.246 | 604.298 | 4.4 | 2324 |
| mobs-25k-mod | 25000 | 0 | 291.320 | 480.704 | 597.849 | 631.214 | 3.4 | 1382 |
| spread-100k-mod | 100000 | 0 | 231.098 | 346.272 | 476.368 | 703.111 | 4.3 | 2765 |
| spread-100k | 100000 | 0 | 418.010 | 506.061 | 753.954 | 857.723 | 2.4 | 2112 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

