//! `Request`/`AgentError`: the exhaustive ADT for capability-denial and
//! other agent-level failures (chassis design §5's citation of the DDT
//! framework's `ToolCall`/`ToolError` pattern — `Capability` is this
//! design's `ToolCall`, `AgentError` is its `ToolError`).

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A concrete ask an agent's output makes at the host-call boundary —
/// mirrors `Capability`'s shape one-for-one so `Capability::allows` can
/// match a grant against a request (see `enforce.rs`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Request {
    ShellExec(String),
    FileRead(String),
    FileWrite(String),
}

/// Exhaustive ADT for agent-level failures (chassis design §5's
/// `ToolCall`/`ToolError` pattern, carried up one layer). Adding a failure
/// mode requires adding a variant here — no catch-all.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum AgentError {
    /// The request was outside the agent's granted `Vec<Capability>`. The
    /// original request is preserved for diagnostics.
    #[error("capability denied: {0:?}")]
    CapabilityDenied(Request),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capability_denied_preserves_the_request_for_diagnostics() {
        let req = Request::ShellExec("rm -rf /".to_string());
        let err = AgentError::CapabilityDenied(req.clone());
        let AgentError::CapabilityDenied(preserved) = err;
        assert_eq!(preserved, req);
    }
}
