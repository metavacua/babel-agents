//! `agent-core`: `Capability` enum, `SandboxConfig`/`AgentError`, and the
//! capability-enforcement pattern shared by every other crate in the
//! `babel-agents` workspace (chassis design §5-8).
//!
//! Task 0 skeleton: no real modules yet — added task-by-task in the
//! chassis bootstrap plan (Tasks 1-3).

pub mod capability;
pub mod enforce;
pub mod error;
pub mod sandbox;

