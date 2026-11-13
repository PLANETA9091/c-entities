# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.419 | 111.737 | 169.186 | 203.853 | 15.1 | 1936 |
| mobs-25k | 25000 | 0 | 495.501 | 576.129 | 640.101 | 746.944 | 2.0 | 1820 |
| mixed-150k | 150000 | 0 | 588.429 | 791.100 | 1009.257 | 1134.686 | 1.7 | 2014 |
| items-100k-mod | 100000 | 0 | 28.089 | 42.273 | 96.802 | 177.650 | 20.0 | 1397 |
| mixed-150k-mod | 150000 | 0 | 159.302 | 271.484 | 416.222 | 541.580 | 6.3 | 2143 |
| mobs-25k-mod | 25000 | 0 | 2946.696 | 3154.238 | 3303.571 | 3452.325 | 0.3 | 2202 |
| mobs-25k-native-mod | 25000 | 0 | 47.189 | 94.221 | 219.467 | 239.434 | 20.0 | 1427 |
| mobs-25k-native | 25000 | 0 | 477.018 | 554.007 | 650.368 | 762.473 | 2.1 | 1996 |
| spread-100k-mod | 100000 | 0 | 190.329 | 263.889 | 390.554 | 478.226 | 5.3 | 2110 |
| spread-100k | 100000 | 0 | 449.979 | 531.080 | 650.361 | 783.799 | 2.2 | 2059 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

