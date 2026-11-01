# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.113 | 78.777 | 141.553 | 194.648 | 15.6 | 2057 |
| mobs-25k | 25000 | 0 | 565.704 | 628.309 | 716.056 | 775.748 | 1.8 | 1642 |
| mixed-150k | 150000 | 0 | 742.141 | 879.735 | 1171.937 | 1244.363 | 1.3 | 2476 |
| items-100k-mod | 100000 | 0 | 31.805 | 76.410 | 136.598 | 172.721 | 20.0 | 1347 |
| mixed-150k-mod | 150000 | 0 | 262.118 | 374.595 | 499.650 | 581.446 | 3.8 | 1646 |
| mobs-25k-mod | 25000 | 0 | 311.801 | 509.210 | 621.501 | 742.826 | 3.2 | 1616 |
| spread-100k-mod | 100000 | 0 | 381.286 | 528.287 | 727.362 | 946.873 | 2.6 | 2033 |
| spread-100k | 100000 | 0 | 769.770 | 905.589 | 1371.008 | 1542.763 | 1.3 | 1920 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

