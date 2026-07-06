//! Stub crate — `agent-orchestrator` (chassis design §3/§8). Implementation deferred to a
//! later B-series plan (B2/B3/B4/B5). This crate exists only so the workspace
//! is structurally complete and `cargo build --workspace` succeeds.
//!
//! `no_std`: the dispatch/ratchet logic itself must attempt to compile
//! against `wasm32v1-none`; real wasmi sandbox execution (which cannot
//! target `wasm32v1-none` — it's the host-side runtime) belongs behind the
//! non-default `runtime` feature gate declared in this crate's Cargo.toml,
//! not in the default build.
//!
//! `cfg_attr(not(test), no_std)`, not bare `no_std`: matches `agent-core`
//! — a bare `#![no_std]` would break `cargo test` once this crate gains
//! real tests (`E0433: cannot find crate std`).
#![cfg_attr(not(test), no_std)]
