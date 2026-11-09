# Release readiness

The development daemon is runnable on the verified incremental Windows toolchain. Production release readiness remains open.

| Gates | Evidence still required |
|---|---|
| G0 | Formal architecture and contract review |
| G1, G8 | Reproducible signed artifacts, installer validation, SBOM and dependency/license audit |
| G2, G3 | Hard resource enforcement under pressure and real inference parity |
| G4, G6 | Grounding and pack provenance tests and reproducible domain-quality reports |
| G5 | Agent sandbox and approval security tests |
| G7 | Fresh Windows/Linux E2E, backup/restore and update rollback |
