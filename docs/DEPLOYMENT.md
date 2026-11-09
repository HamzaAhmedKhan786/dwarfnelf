# Packaging, Installation and Operations
## Distribution
Single signed Windows installer (desktop+daemon+CLI+native runtime) and Linux package/AppImage as supported; macOS packaging and GPU backends require platform-specific certification. Optional daemon-only artifact for headless users. No pip/npm/Rust/Docker setup for end users. Model weights and knowledge packs can be downloaded separately with explicit consent and verified hashes.
## Lifecycle
Install → provision local credential → start daemon → UI discovery/version handshake → first-run hardware wizard → model/import → pack/index → chat. Daemon survives UI exit by user policy. Update flow: verify signature, stop/drain, backup, migrate, atomic replace, health check, rollback on failure. Uninstall offers keep/delete models, indexes and private documents.
## CI/CD
Formatting, lint, Rust unit/integration, C++ sanitizer/fuzz, JS tests, Python research tests, contract tests, SBOM/license, vulnerability scans, installer smoke tests, offline first-run E2E, signed reproducible release artifacts. Windows/Linux CPU baseline mandatory; GPU matrix on dedicated runners.
## API discovery
Desktop checks authenticated local config and `/health`, `/v1/capabilities`; never blindly trust an arbitrary process on a port. Compatibility policy: versioned semantic API, explicit feature flags, migration and rollback.
