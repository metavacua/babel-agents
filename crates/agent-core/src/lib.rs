//! `agent-core`: `Capability` enum, `SandboxConfig`/`AgentError`, and the
//! capability-enforcement pattern shared by every other crate in the
//! `babel-agents` workspace (chassis design §5-8).
//!
//! `no_std` + `alloc`: every crate in this workspace must attempt to
//! compile against `wasm32v1-none`, which has no `std`. `String`/`Vec`
//! still work via `alloc`; anything needing OS/host access does not belong
//! in this crate.
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod capability;
pub mod enforce;
pub mod error;
pub mod sandbox;
