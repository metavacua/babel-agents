//! `execute_request`: the runtime capability check at the host-call
//! boundary (chassis design §6), plus the `CanCommit`-style marker-trait
//! pattern for the compile-time layer. Both layers are required — trait
//! bounds catch orchestrator-code mistakes at compile time; the runtime
//! check is what actually holds even if an agent's output tries to request
//! something outside its grant (design §6).

use std::marker::PhantomData;

use crate::capability::Capability;
use crate::error::{AgentError, Request};

/// Marker type for the generator agent class (chassis design §3, class 2:
/// no capabilities at all — pure function).
pub struct GeneratorClass;

/// Marker type for the committer agent class (chassis design §3, class 8:
/// `ShellExec("git commit *")`, `ShellExec("git push *")`).
pub struct CommitterClass;

/// A handle to one running agent instance: its capability grant plus a
/// compile-time class marker. `Class` carries no data — it exists purely so
/// the compiler can select which marker traits (`CanCommit`, etc.) are
/// implemented for which class, per chassis design §6's compile-time layer.
pub struct AgentHandle<Class> {
    pub capabilities: Vec<Capability>,
    _class: PhantomData<Class>,
}

impl<Class> AgentHandle<Class> {
    pub fn new(capabilities: Vec<Capability>) -> Self {
        Self {
            capabilities,
            _class: PhantomData,
        }
    }
}

/// Minimal stand-in for whatever an executed request actually produces.
/// `agent-orchestrator` (B5) owns the real artifact-flow types (design §9);
/// this crate only needs to demonstrate the enforcement contract.
pub type Output = String;

/// Runtime capability check at the host-call boundary (chassis design §6).
/// Deny-by-default: a request is only performed if *some* capability in the
/// agent's grant allows it; otherwise `AgentError::CapabilityDenied` is
/// returned with the original request preserved.
pub fn execute_request<Class>(
    agent: &AgentHandle<Class>,
    request: &Request,
) -> Result<Output, AgentError> {
    if !agent.capabilities.iter().any(|c| c.allows(request)) {
        return Err(AgentError::CapabilityDenied(request.clone()));
    }
    Ok(format!("executed: {request:?}"))
}

/// Compile-time layer (chassis design §6): only `AgentHandle<CommitterClass>`
/// implements this trait, so `AgentHandle<GeneratorClass>::git_commit` is a
/// compiler error, not a runtime one — see the `compile_fail` doctest below.
pub trait CanCommit {
    fn git_commit(&self, msg: &str) -> Result<(), AgentError>;
}

impl CanCommit for AgentHandle<CommitterClass> {
    fn git_commit(&self, msg: &str) -> Result<(), AgentError> {
        let request = Request::ShellExec(format!("git commit -m {msg:?}"));
        execute_request(self, &request)?;
        Ok(())
    }
}

/// Positive control: `AgentHandle<CommitterClass>` granted the matching
/// `ShellExec` capability *can* call `git_commit` — this compiles and
/// succeeds at runtime, showing the trait-bound layer correctly admits the
/// class it's meant for (not just correctly rejecting the wrong one, see
/// the compile_fail doctest below for that half).
///
/// ```
/// use agent_core::capability::Capability;
/// use agent_core::enforce::{AgentHandle, CanCommit, CommitterClass};
///
/// let agent = AgentHandle::<CommitterClass>::new(vec![Capability::ShellExec(
///     "git commit *".to_string(),
/// )]);
/// assert!(agent.git_commit("chassis bootstrap").is_ok());
/// ```
///
/// Negative case: `AgentHandle<GeneratorClass>` has no `CanCommit` impl, so
/// calling `git_commit` on it must fail to COMPILE (E0599: no method named
/// `git_commit`), not panic at runtime.
///
/// ```compile_fail
/// use agent_core::enforce::{AgentHandle, CanCommit, GeneratorClass};
///
/// let agent = AgentHandle::<GeneratorClass>::new(vec![]);
/// let _ = agent.git_commit("should not compile");
/// ```
pub struct CanCommitDoctestAnchor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::Capability;
    use crate::error::{AgentError, Request};

    /// Runtime layer, negative case: a request outside the agent's grant
    /// must be denied. Constructed so a naive "always return Ok"
    /// implementation fails this test — the request (`ShellExec`) is not
    /// merely absent from the grant, it's a *different variant* than the
    /// one variant granted (`FileRead`), so this also fails against a
    /// naive "any non-empty capability list allows everything" stub.
    #[test]
    fn execute_request_denies_out_of_grant_shell_exec() {
        let agent = AgentHandle::<GeneratorClass>::new(vec![Capability::FileRead(
            "docs/**".to_string(),
        )]);
        let request = Request::ShellExec("git push origin main".to_string());

        let result = execute_request(&agent, &request);

        match result {
            Err(AgentError::CapabilityDenied(denied)) => assert_eq!(denied, request),
            other => panic!("expected CapabilityDenied, got {other:?}"),
        }
    }

    /// Runtime layer, positive case: the dual of the test above. Without
    /// this, a naive "always return Err(CapabilityDenied)" implementation
    /// would also pass the negative test above — this is what rules that
    /// out (issue #206's principle: a predicate that holds of every input
    /// carries no information, so the check must be shown to accept
    /// *something* too).
    #[test]
    fn execute_request_allows_in_grant_file_read() {
        let agent = AgentHandle::<GeneratorClass>::new(vec![Capability::FileRead(
            "docs/**".to_string(),
        )]);
        let request = Request::FileRead("docs/design.md".to_string());

        let result = execute_request(&agent, &request);

        assert!(result.is_ok(), "expected Ok, got {result:?}");
    }
}
