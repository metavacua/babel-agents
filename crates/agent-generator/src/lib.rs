//! Stub crate — `agent-generator` (chassis design §3/§8). Implementation deferred to a
//! later B-series plan (B2/B3/B4/B5). This crate exists only so the workspace
//! is structurally complete and `cargo build --workspace` succeeds.
//!
//! `no_std`: content-generating agent classes are pure functions with zero
//! I/O (chassis design §5) and must attempt to compile against
//! `wasm32v1-none`.
//!
//! `cfg_attr(not(test), no_std)`, not bare `no_std`: matches `agent-core`
//! — a bare `#![no_std]` would break `cargo test` once this crate gains
//! real tests (`E0433: cannot find crate std`).
#![cfg_attr(not(test), no_std)]
