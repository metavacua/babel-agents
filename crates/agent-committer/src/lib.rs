//! Stub crate — `agent-committer` (chassis design §3/§8). Implementation deferred to a
//! later B-series plan (B2/B3/B4/B5). This crate exists only so the workspace
//! is structurally complete and `cargo build --workspace` succeeds.
//!
//! `no_std`: the committer's own decision logic (was every validator
//! satisfied?) must attempt to compile against `wasm32v1-none`; the actual
//! `git commit`/`git push` invocation is a separate, real-OS-access
//! concern that belongs behind a feature gate, not in this crate's default
//! build (see the workspace CI matrix).
//!
//! `cfg_attr(not(test), no_std)`, not bare `no_std`: matches `agent-core`
//! — a bare `#![no_std]` would break `cargo test` once this crate gains
//! real tests (`E0433: cannot find crate std`), since `#[cfg(test)]` alone
//! doesn't disable it.
#![cfg_attr(not(test), no_std)]
