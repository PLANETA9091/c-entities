# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.869 | 80.506 | 149.342 | 227.570 | 15.0 | 1730 |
| mobs-25k | 25000 | 0 | 547.489 | 605.657 | 670.752 | 730.255 | 1.8 | 960 |
| mixed-150k | 150000 | 0 | 614.251 | 742.314 | 1031.382 | 1116.722 | 1.6 | 3260 |
| items-100k-mod | 100000 | 0 | 26.623 | 75.644 | 140.760 | 186.326 | 20.0 | 1372 |
| mixed-150k-mod | 150000 | 0 | 297.552 | 412.242 | 585.568 | 712.922 | 3.4 | 1766 |
| mobs-25k-mod | 25000 | 0 | 269.556 | 423.208 | 542.266 | 616.413 | 3.7 | 1730 |
| spread-100k-mod | 100000 | 0 | 273.334 | 418.532 | 513.500 | 685.454 | 3.7 | 3017 |
| spread-100k | 100000 | 0 | 454.548 | 539.322 | 764.083 | 900.293 | 2.2 | 2091 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

