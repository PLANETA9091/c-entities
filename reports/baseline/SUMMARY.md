# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.051 | 77.229 | 145.628 | 212.485 | 15.6 | 1274 |
| mobs-25k | 25000 | 0 | 273.636 | 311.748 | 342.897 | 359.478 | 3.7 | 1412 |
| mixed-150k | 150000 | 0 | 487.244 | 609.067 | 872.640 | 1063.844 | 2.1 | 2838 |
| items-100k-mod | 100000 | 0 | 13.081 | 17.839 | 96.579 | 151.651 | 20.0 | 1287 |
| mixed-150k-mod | 150000 | 0 | 270.848 | 326.285 | 376.111 | 450.458 | 3.7 | 1536 |
| mobs-25k-mod | 25000 | 0 | 138.499 | 201.524 | 256.946 | 298.602 | 7.2 | 1749 |
| spread-100k-mod | 100000 | 0 | 152.723 | 231.443 | 299.435 | 392.599 | 6.5 | 2187 |
| spread-100k | 100000 | 0 | 416.528 | 479.181 | 698.713 | 726.911 | 2.4 | 2230 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

