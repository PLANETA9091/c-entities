# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 78.749 | 128.916 | 196.646 | 285.697 | 12.7 | 1807 |
| mobs-25k | 25000 | 0 | 564.852 | 657.395 | 757.845 | 982.798 | 1.8 | 1639 |
| mixed-150k | 150000 | 0 | 519.359 | 755.988 | 1008.739 | 1192.182 | 1.9 | 2041 |
| items-100k-mod | 100000 | 0 | 31.932 | 49.060 | 121.479 | 242.174 | 20.0 | 1451 |
| mixed-150k-mod | 150000 | 0 | 743.533 | 883.812 | 1114.874 | 1266.474 | 1.3 | 1950 |
| mobs-25k-mod | 25000 | 0 | 680.079 | 823.292 | 1116.207 | 1211.615 | 1.5 | 2163 |
| spread-100k-mod | 100000 | 0 | 643.064 | 754.774 | 993.390 | 1280.723 | 1.6 | 3058 |
| spread-100k | 100000 | 0 | 485.020 | 583.170 | 805.417 | 893.700 | 2.1 | 2234 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

