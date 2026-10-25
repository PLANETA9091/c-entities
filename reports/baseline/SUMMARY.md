# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 58.298 | 73.464 | 131.328 | 180.499 | 17.2 | 1286 |
| mobs-25k | 25000 | 0 | 382.180 | 430.377 | 468.831 | 509.080 | 2.6 | 1826 |
| mixed-150k | 150000 | 0 | 398.590 | 484.848 | 732.935 | 911.856 | 2.5 | 2405 |
| items-100k-mod | 100000 | 0 | 16.509 | 19.805 | 55.174 | 108.792 | 20.0 | 1114 |
| mixed-150k-mod | 150000 | 0 | 357.246 | 406.604 | 465.734 | 522.271 | 2.8 | 2874 |
| mobs-25k-mod | 25000 | 0 | 205.631 | 245.885 | 278.827 | 313.322 | 4.9 | 1551 |
| spread-100k-mod | 100000 | 0 | 273.195 | 326.502 | 415.975 | 449.465 | 3.7 | 2680 |
| spread-100k | 100000 | 0 | 425.459 | 498.991 | 711.292 | 776.754 | 2.4 | 2281 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

