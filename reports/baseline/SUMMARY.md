# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.382 | 77.448 | 143.401 | 234.052 | 16.0 | 938 |
| mobs-25k | 25000 | 0 | 397.960 | 447.891 | 507.102 | 548.362 | 2.5 | 1706 |
| mixed-150k | 150000 | 0 | 756.770 | 1026.541 | 1507.749 | 1821.818 | 1.3 | 2022 |
| items-100k-mod | 100000 | 0 | 33.244 | 92.574 | 147.435 | 212.638 | 20.0 | 1292 |
| mixed-150k-mod | 150000 | 0 | 547.416 | 708.119 | 996.386 | 1166.865 | 1.8 | 1762 |
| mobs-25k-mod | 25000 | 0 | 199.303 | 319.339 | 526.830 | 606.818 | 5.0 | 1496 |
| spread-100k-mod | 100000 | 0 | 183.061 | 319.296 | 475.487 | 660.125 | 5.5 | 2336 |
| spread-100k | 100000 | 0 | 489.396 | 551.677 | 755.795 | 781.674 | 2.0 | 2191 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

