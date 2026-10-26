# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 62.813 | 77.863 | 143.580 | 184.716 | 15.9 | 1227 |
| mobs-25k | 25000 | 0 | 266.038 | 303.746 | 351.348 | 394.128 | 3.8 | 1887 |
| mixed-150k | 150000 | 0 | 549.819 | 657.577 | 927.688 | 1045.003 | 1.8 | 1836 |
| items-100k-mod | 100000 | 0 | 14.115 | 18.185 | 78.393 | 125.331 | 20.0 | 1334 |
| mixed-150k-mod | 150000 | 0 | 286.749 | 349.419 | 406.139 | 482.451 | 3.5 | 1786 |
| mobs-25k-mod | 25000 | 0 | 137.076 | 246.498 | 323.273 | 381.954 | 7.3 | 1118 |
| spread-100k-mod | 100000 | 0 | 139.529 | 232.485 | 324.567 | 409.558 | 7.2 | 2314 |
| spread-100k | 100000 | 0 | 396.505 | 453.660 | 649.386 | 674.241 | 2.5 | 2144 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

