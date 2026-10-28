# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 69.166 | 87.767 | 151.541 | 246.561 | 14.5 | 830 |
| mobs-25k | 25000 | 0 | 554.876 | 616.675 | 665.262 | 700.399 | 1.8 | 1590 |
| mixed-150k | 150000 | 0 | 635.838 | 768.944 | 1069.750 | 1169.043 | 1.6 | 2536 |
| items-100k-mod | 100000 | 0 | 31.278 | 87.306 | 156.220 | 197.992 | 20.0 | 1432 |
| mixed-150k-mod | 150000 | 0 | 442.292 | 558.704 | 823.990 | 901.442 | 2.3 | 1846 |
| mobs-25k-mod | 25000 | 0 | 227.553 | 376.406 | 598.786 | 637.283 | 4.4 | 1632 |
| spread-100k-mod | 100000 | 0 | 182.834 | 317.941 | 474.218 | 604.821 | 5.5 | 2721 |
| spread-100k | 100000 | 0 | 431.786 | 487.844 | 706.075 | 749.186 | 2.3 | 2193 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

