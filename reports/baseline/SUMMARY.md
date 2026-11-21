# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 78.234 | 136.784 | 201.049 | 261.627 | 12.8 | 2222 |
| mobs-25k | 25000 | 0 | 535.059 | 609.978 | 658.642 | 822.445 | 1.9 | 1372 |
| mixed-150k | 150000 | 0 | 475.989 | 636.134 | 806.880 | 905.357 | 2.1 | 3041 |
| items-100k-mod | 100000 | 0 | 19.390 | 39.421 | 114.028 | 275.173 | 20.0 | 1161 |
| mixed-150k-mod | 150000 | 0 | 56.346 | 93.003 | 223.812 | 348.287 | 17.7 | 1885 |
| mobs-25k-mod | 25000 | 0 | 3024.293 | 3667.094 | 3951.340 | 4160.497 | 0.3 | 1335 |
| mobs-25k-native-mod | 25000 | 0 | 17.748 | 30.480 | 109.734 | 146.227 | 20.0 | 1243 |
| mobs-25k-native | 25000 | 0 | 314.688 | 397.900 | 493.256 | 546.470 | 3.2 | 1134 |
| spread-100k-mod | 100000 | 0 | 49.379 | 81.758 | 204.345 | 299.909 | 20.0 | 1950 |
| spread-100k | 100000 | 0 | 521.868 | 615.695 | 715.494 | 829.578 | 1.9 | 1792 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

