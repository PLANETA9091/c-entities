# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.019 | 83.594 | 155.596 | 288.344 | 14.9 | 749 |
| mobs-25k | 25000 | 0 | 497.335 | 547.197 | 590.785 | 611.556 | 2.0 | 931 |
| mixed-150k | 150000 | 0 | 598.590 | 744.681 | 1060.894 | 1212.860 | 1.7 | 2790 |
| items-100k-mod | 100000 | 0 | 35.412 | 88.988 | 158.461 | 215.893 | 20.0 | 1553 |
| mixed-150k-mod | 150000 | 0 | 317.581 | 415.024 | 540.623 | 607.036 | 3.1 | 2204 |
| mobs-25k-mod | 25000 | 0 | 159.191 | 348.202 | 640.133 | 801.787 | 6.3 | 2258 |
| spread-100k-mod | 100000 | 0 | 167.816 | 290.346 | 456.839 | 555.795 | 6.0 | 1865 |
| spread-100k | 100000 | 0 | 437.051 | 495.117 | 692.333 | 744.584 | 2.3 | 1852 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

