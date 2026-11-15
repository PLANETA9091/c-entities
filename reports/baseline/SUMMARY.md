# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 73.487 | 129.805 | 197.424 | 260.194 | 13.6 | 1652 |
| mobs-25k | 25000 | 0 | 581.867 | 681.754 | 828.303 | 949.649 | 1.7 | 1910 |
| mixed-150k | 150000 | 0 | 554.542 | 775.153 | 948.148 | 1099.161 | 1.8 | 3384 |
| items-100k-mod | 100000 | 0 | 35.249 | 57.940 | 186.802 | 219.398 | 20.0 | 1705 |
| mixed-150k-mod | 150000 | 0 | 93.162 | 146.234 | 303.060 | 354.622 | 10.7 | 2131 |
| mobs-25k-mod | 25000 | 0 | 2743.791 | 2986.228 | 3217.619 | 3298.839 | 0.4 | 2242 |
| mobs-25k-native-mod | 25000 | 0 | 26.547 | 47.115 | 121.257 | 255.627 | 20.0 | 1679 |
| mobs-25k-native | 25000 | 0 | 537.751 | 615.684 | 737.083 | 874.303 | 1.9 | 1623 |
| spread-100k-mod | 100000 | 0 | 62.507 | 127.077 | 222.067 | 360.723 | 16.0 | 2242 |
| spread-100k | 100000 | 0 | 518.171 | 616.931 | 704.669 | 868.814 | 1.9 | 2254 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

