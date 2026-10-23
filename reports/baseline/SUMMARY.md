# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 85.953 | 104.822 | 142.777 | 205.674 | 11.6 | 1087 |
| mobs-25k | 25000 | 0 | 398.798 | 450.244 | 498.612 | 514.666 | 2.5 | 1530 |
| mixed-150k | 150000 | 0 | 557.570 | 610.066 | 670.718 | 771.684 | 1.8 | 3110 |
| items-100k-mod | 100000 | 0 | 0.595 | 0.754 | 1.119 | 8.442 | 20.0 | 1621 |
| mixed-150k-mod | 150000 | 0 | 324.788 | 391.062 | 567.250 | 628.298 | 3.1 | 1974 |
| mobs-25k-mod | 25000 | 0 | 397.854 | 454.958 | 512.677 | 553.915 | 2.5 | 1420 |
| spread-100k-mod | 100000 | 0 | 454.467 | 524.197 | 615.817 | 735.054 | 2.2 | 2871 |
| spread-100k | 100000 | 0 | 445.724 | 523.069 | 609.210 | 711.054 | 2.2 | 2783 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

