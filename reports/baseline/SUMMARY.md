# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 67.183 | 113.177 | 185.559 | 246.881 | 14.9 | 2435 |
| mobs-25k | 25000 | 0 | 540.317 | 609.881 | 703.897 | 812.006 | 1.9 | 1494 |
| mixed-150k | 150000 | 0 | 669.449 | 952.542 | 1326.055 | 1536.455 | 1.5 | 2336 |
| items-100k-mod | 100000 | 0 | 28.149 | 46.164 | 112.306 | 176.948 | 20.0 | 1385 |
| mixed-150k-mod | 150000 | 0 | 836.264 | 984.729 | 1239.205 | 1403.165 | 1.2 | 2042 |
| mobs-25k-mod | 25000 | 0 | 640.218 | 844.700 | 1033.856 | 1192.144 | 1.6 | 1651 |
| spread-100k-mod | 100000 | 0 | 690.500 | 784.521 | 1047.748 | 1313.883 | 1.4 | 3248 |
| spread-100k | 100000 | 0 | 491.801 | 589.003 | 811.652 | 917.999 | 2.0 | 2410 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

