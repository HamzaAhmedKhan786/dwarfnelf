# Dependency blockers

- **Fresh Windows builds.** Code Integrity event 3077 under `VerifiedAndReputableDesktop` confirmed blocks of locally generated `windows_interface-*.dll` and `async_trait-*.dll` files, as well as some test executables. A prior fresh build exited 101 after rustc reported E0463. Incremental successes cover the tested worktree and cache. Run clean native checks on authorized Windows CI or a Windows build machine and Linux portability checks in an approved Linux environment. WSL enumeration from the current sandbox returned access denied.
- **Inference fixture.** The CPU E2E needs a pinned, verified llama.cpp worker and explicit consent for a small GGUF download, or a user-provided GGUF with checksum and license.
- **Release verification.** A trusted Windows signing identity, audited native dependency inventory and platform installers are pending. Signing the final daemon alone would leave build-time DLL verification unresolved.
