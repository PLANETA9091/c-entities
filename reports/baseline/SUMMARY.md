# Baseline profile of Paper 1.21.8 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| control | 0 | 0 | 0.275 | 0.350 | 0.516 | 4.782 | 20.0 | 876 |
| items-25k | 25000 | 0 | 19.048 | 22.037 | 30.734 | 42.772 | 20.0 | 774 |
| items-100k | 100000 | 0 | 65.205 | 72.138 | 109.021 | 139.155 | 15.3 | 1368 |
| mobs-10k | 10000 | 0 | 114.455 | 121.018 | 154.260 | 165.783 | 8.7 | 1653 |
| mobs-25k | 25000 | 0 | 440.557 | 483.441 | 532.153 | 564.344 | 2.3 | 1859 |
| mixed-50k | 50000 | 0 | 128.808 | 144.644 | 176.022 | 189.297 | 7.8 | 931 |
| mixed-150k | 150000 | 0 | 676.202 | 720.070 | 767.849 | 796.980 | 1.5 | 2815 |
| tiles-25k | 0 | 24964 | 6.427 | 8.167 | 10.467 | 25.042 | 20.0 | 614 |
| tiles-100k | 0 | 68121 | 27.634 | 28.697 | 30.896 | 51.152 | 20.0 | 1042 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

