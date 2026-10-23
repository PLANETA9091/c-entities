# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 58.424 | 73.753 | 131.468 | 171.033 | 17.1 | 1019 |
| mobs-25k | 25000 | 0 | 384.411 | 430.157 | 473.132 | 510.559 | 2.6 | 950 |
| mixed-150k | 150000 | 0 | 439.953 | 523.420 | 743.389 | 819.671 | 2.3 | 2503 |
| items-100k-mod | 100000 | 0 | 61.051 | 87.402 | 153.767 | 239.967 | 16.4 | 1144 |
| mixed-150k-mod | 150000 | 0 | 158.606 | 214.670 | 318.902 | 393.294 | 6.3 | 1740 |
| mobs-25k-mod | 25000 | 0 | 117.858 | 183.640 | 270.298 | 350.507 | 8.5 | 1873 |
| spread-100k-mod | 100000 | 0 | 222.852 | 279.918 | 388.157 | 447.142 | 4.5 | 2039 |
| spread-100k | 100000 | 0 | 425.915 | 490.484 | 716.732 | 793.778 | 2.3 | 2109 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

