# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 64.826 | 78.827 | 144.812 | 201.658 | 15.4 | 842 |
| mobs-25k | 25000 | 0 | 537.230 | 611.542 | 700.024 | 888.577 | 1.9 | 1316 |
| mixed-150k | 150000 | 0 | 560.711 | 668.203 | 915.339 | 1055.087 | 1.8 | 2286 |
| items-100k-mod | 100000 | 0 | 26.817 | 75.552 | 134.942 | 181.323 | 20.0 | 1445 |
| mixed-150k-mod | 150000 | 0 | 253.024 | 354.512 | 534.928 | 673.640 | 4.0 | 1863 |
| mobs-25k-mod | 25000 | 0 | 304.340 | 566.959 | 766.628 | 815.343 | 3.3 | 1201 |
| spread-100k-mod | 100000 | 0 | 274.529 | 426.692 | 558.549 | 758.678 | 3.6 | 2346 |
| spread-100k | 100000 | 0 | 422.942 | 480.616 | 676.303 | 716.326 | 2.4 | 2212 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

