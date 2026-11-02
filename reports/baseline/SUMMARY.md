# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 77.000 | 96.489 | 170.830 | 293.435 | 13.0 | 2052 |
| mobs-25k | 25000 | 0 | 527.205 | 594.951 | 653.876 | 722.314 | 1.9 | 920 |
| mixed-150k | 150000 | 0 | 575.884 | 706.002 | 1018.308 | 1098.138 | 1.7 | 2711 |
| items-100k-mod | 100000 | 0 | 28.817 | 72.150 | 146.644 | 193.582 | 20.0 | 1236 |
| mixed-150k-mod | 150000 | 0 | 223.855 | 322.904 | 452.894 | 586.557 | 4.5 | 2389 |
| mobs-25k-mod | 25000 | 0 | 294.132 | 532.843 | 655.352 | 786.088 | 3.4 | 1449 |
| spread-100k-mod | 100000 | 0 | 276.155 | 451.446 | 589.505 | 810.784 | 3.6 | 2234 |
| spread-100k | 100000 | 0 | 513.804 | 579.346 | 807.518 | 1017.214 | 1.9 | 2183 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

