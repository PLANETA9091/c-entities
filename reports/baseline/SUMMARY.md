# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.484 | 116.072 | 170.469 | 202.983 | 14.4 | 943 |
| mobs-25k | 25000 | 0 | 513.187 | 583.343 | 640.220 | 804.001 | 1.9 | 1294 |
| mixed-150k | 150000 | 0 | 467.436 | 627.954 | 850.309 | 936.304 | 2.1 | 3017 |
| items-100k-mod | 100000 | 0 | 25.048 | 41.281 | 102.427 | 262.141 | 20.0 | 1631 |
| mixed-150k-mod | 150000 | 0 | 174.395 | 311.897 | 490.006 | 695.868 | 5.7 | 2450 |
| mobs-25k-mod | 25000 | 0 | 2506.258 | 2699.731 | 2855.665 | 3010.719 | 0.4 | 1912 |
| mobs-25k-native-mod | 25000 | 0 | 25.217 | 48.987 | 121.300 | 164.398 | 20.0 | 1503 |
| mobs-25k-native | 25000 | 0 | 398.350 | 467.402 | 578.067 | 658.793 | 2.5 | 1100 |
| spread-100k-mod | 100000 | 0 | 190.522 | 290.365 | 449.806 | 492.433 | 5.2 | 2011 |
| spread-100k | 100000 | 0 | 442.359 | 553.593 | 692.231 | 768.800 | 2.3 | 1941 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

