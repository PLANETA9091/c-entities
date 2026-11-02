# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 44.959 | 56.722 | 118.476 | 159.448 | 20.0 | 939 |
| mobs-25k | 25000 | 0 | 528.763 | 587.768 | 643.263 | 681.834 | 1.9 | 1122 |
| mixed-150k | 150000 | 0 | 436.361 | 523.644 | 722.647 | 810.401 | 2.3 | 2473 |
| items-100k-mod | 100000 | 0 | 21.304 | 62.067 | 113.604 | 176.534 | 20.0 | 1277 |
| mixed-150k-mod | 150000 | 0 | 234.705 | 326.214 | 463.568 | 522.233 | 4.3 | 1616 |
| mobs-25k-mod | 25000 | 0 | 332.455 | 631.912 | 779.442 | 918.436 | 3.0 | 1290 |
| spread-100k-mod | 100000 | 0 | 268.209 | 406.163 | 514.204 | 676.416 | 3.7 | 1827 |
| spread-100k | 100000 | 0 | 398.305 | 463.604 | 681.314 | 796.368 | 2.5 | 2116 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

