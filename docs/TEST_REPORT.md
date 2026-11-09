# Test report

Verification environment is Windows 11 x86_64 MSVC with Cargo/rustc 1.96.1 and local build artifacts. The baseline before this milestone was `87cb010`. The checks below cover the tested backend snapshot.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | Passed |
| `cargo clippy --locked --offline --workspace --all-targets -- -D warnings` | Passed |
| `cargo test --locked --offline --workspace` | Nine tests passed. Windows blocked launch of the workspace storage test binary with Win32 error 4551. |
| `cargo test --locked --offline -p mama-storage` | Storage test passed in the package-scoped build, including v1-to-v2 upgrade, persistence, retention and future-schema rejection. |
| `./scripts/smoke-daemon.ps1` | Build and native loopback checks passed with socket access. Covered import/search, CPU admission and redacted run-history lookup. |
| OpenAPI validator | Passed |
| `python -m compileall -q scripts` and `python scripts/development_progress.py` | Passed |

The prior fresh native Windows build was blocked by Smart App Control. No fresh build pass is recorded for this increment. See [dependency blockers](DEPENDENCY_BLOCKERS.md) for the verified failure mode.
