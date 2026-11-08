# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 71.222 | 119.039 | 178.717 | 226.990 | 14.0 | 2498 |
| mobs-25k | 25000 | 0 | 357.988 | 431.853 | 524.842 | 672.151 | 2.8 | 1076 |
| mixed-150k | 150000 | 0 | 569.956 | 819.012 | 1094.150 | 1235.176 | 1.8 | 2017 |
| items-100k-mod | 100000 | 0 | 29.576 | 45.849 | 109.591 | 198.992 | 20.0 | 1606 |
| mixed-150k-mod | 150000 | 0 | 829.999 | 952.640 | 1200.859 | 1313.042 | 1.2 | 2290 |
| mobs-25k-mod | 25000 | 0 | 435.719 | 655.104 | 788.729 | 895.640 | 2.3 | 1833 |
| spread-100k-mod | 100000 | 0 | 369.652 | 507.226 | 694.873 | 857.680 | 2.7 | 3200 |
| spread-100k | 100000 | 0 | 340.908 | 428.363 | 587.105 | 664.935 | 2.9 | 2592 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

