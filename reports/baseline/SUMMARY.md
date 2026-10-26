# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 66.271 | 82.350 | 154.875 | 201.680 | 15.1 | 1496 |
| mobs-25k | 25000 | 0 | 316.194 | 356.999 | 394.652 | 419.550 | 3.2 | 1537 |
| mixed-150k | 150000 | 0 | 417.933 | 522.258 | 779.366 | 831.782 | 2.4 | 2534 |
| items-100k-mod | 100000 | 0 | 14.692 | 19.243 | 91.783 | 249.107 | 20.0 | 1388 |
| mixed-150k-mod | 150000 | 0 | 254.532 | 308.987 | 366.590 | 426.946 | 3.9 | 2606 |
| mobs-25k-mod | 25000 | 0 | 163.603 | 242.399 | 321.900 | 360.283 | 6.1 | 1268 |
| spread-100k-mod | 100000 | 0 | 162.835 | 248.401 | 325.615 | 393.051 | 6.1 | 2570 |
| spread-100k | 100000 | 0 | 422.823 | 482.500 | 749.638 | 802.724 | 2.4 | 1970 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

