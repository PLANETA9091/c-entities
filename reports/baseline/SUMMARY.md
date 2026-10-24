# Baseline profile of Paper 26.3 (no optimizations)

Measured on GitHub Actions (ubuntu-latest, 4 vCPU), JFR enabled (~2-5% overhead, identical across runs).

| run | entities | block entities | MSPT p50 | p90 | p99 | max, ms | TPS est. | heap, MB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| items-100k | 100000 | 0 | 59.669 | 71.646 | 130.459 | 158.695 | 16.8 | 1006 |
| mobs-25k | 25000 | 0 | 401.372 | 467.492 | 534.814 | 596.583 | 2.5 | 1459 |
| mixed-150k | 150000 | 0 | 473.229 | 580.554 | 857.368 | 986.508 | 2.1 | 2753 |
| items-100k-mod | 100000 | 0 | 29.159 | 44.026 | 130.217 | 262.535 | 20.0 | 1386 |
| mixed-150k-mod | 150000 | 0 | 293.987 | 349.482 | 386.713 | 435.910 | 3.4 | 1954 |
| mobs-25k-mod | 25000 | 0 | 124.476 | 291.988 | 379.732 | 480.997 | 8.0 | 1794 |
| spread-100k-mod | 100000 | 0 | 164.658 | 260.508 | 346.945 | 451.337 | 6.1 | 1984 |
| spread-100k | 100000 | 0 | 402.159 | 473.383 | 658.143 | 731.967 | 2.5 | 2170 |

Per-run reports: `reports/baseline/<run>/REPORT.md`; raw data: `baseline.json` in run artifacts.

