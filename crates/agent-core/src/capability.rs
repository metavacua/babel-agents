//! `Capability`: the closed, exhaustive enum of everything an agent instance
//! may be granted permission to do (chassis design §5). Adding a variant
//! requires editing this enum and every match site — the compiler enforces
//! exhaustiveness (see the `compile_fail` doctest below).

use serde::{Deserialize, Serialize};

/// A single granted permission. Closed/exhaustive by design (chassis design
/// §5, Global Constraints): no `Other(String)` escape hatch. Extending this
/// set requires editing this enum and every match site.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Capability {
    /// Shell command matching a fixed pattern, e.g.
    /// `"cargo build --target wasm32v1-none *"`.
    ShellExec(CommandPattern),
    /// Read files matching a glob, e.g. `"docs/**"`.
    FileRead(GlobPattern),
    /// Write files matching a glob.
    FileWrite(GlobPattern),
}

pub type CommandPattern = String;
pub type GlobPattern = String;

impl Capability {
    /// Does this grant allow `request` (chassis design §6)? Used by
    /// `enforce::execute_request` as the runtime capability check at the
    /// host-call boundary — deny-by-default: only an exact-variant,
    /// glob-matching grant returns `true`.
    pub fn allows(&self, request: &crate::error::Request) -> bool {
        use crate::error::Request;
        match (self, request) {
            (Capability::ShellExec(pattern), Request::ShellExec(cmd)) => {
                glob_match(pattern, cmd)
            }
            (Capability::FileRead(pattern), Request::FileRead(path)) => {
                glob_match(pattern, path)
            }
            (Capability::FileWrite(pattern), Request::FileWrite(path)) => {
                glob_match(pattern, path)
            }
            _ => false,
        }
    }
}

/// Minimal glob matcher supporting `*` as "match any sequence, including
/// empty" — enough for command patterns like `"cargo build ... *"` and file
/// globs like `"docs/**"`.
fn glob_match(pattern: &str, text: &str) -> bool {
    fn helper(p: &[u8], t: &[u8]) -> bool {
        match p.first() {
            None => t.is_empty(),
            Some(b'*') => helper(&p[1..], t) || (!t.is_empty() && helper(p, &t[1..])),
            Some(c) => t.first() == Some(c) && helper(&p[1..], &t[1..]),
        }
    }
    helper(pattern.as_bytes(), text.as_bytes())
}

/// Exhaustiveness is compiler-enforced: a `match` over `Capability` that
/// covers only 2 of its 3 variants must fail to compile with
/// `error[E0004]: non-exhaustive patterns`. This is the actual claim being
/// tested here, not a stand-in for "the type exists" — see the sibling
/// doctest just below for confirmation that a *full* match compiles fine,
/// isolating the failure to the missing arm specifically.
///
/// ```compile_fail
/// use agent_core::capability::Capability;
///
/// fn handle(cap: Capability) -> &'static str {
///     match cap {
///         Capability::ShellExec(_) => "shell",
///         Capability::FileRead(_) => "read",
///         // Capability::FileWrite intentionally omitted.
///     }
/// }
/// ```
///
/// Control case: the same match, made exhaustive, compiles fine. This
/// isolates the compile_fail above to the missing arm (E0004), not to some
/// unrelated typo or import error.
///
/// ```
/// use agent_core::capability::Capability;
///
/// fn handle(cap: Capability) -> &'static str {
///     match cap {
///         Capability::ShellExec(_) => "shell",
///         Capability::FileRead(_) => "read",
///         Capability::FileWrite(_) => "write",
///     }
/// }
/// # let _ = handle(Capability::FileRead("x".into()));
/// ```
pub struct ExhaustivenessDoctestAnchor;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Request;

    #[test]
    fn shell_exec_roundtrips_through_json() {
        let cap = Capability::ShellExec("cargo build --target wasm32v1-none *".to_string());
        let json = serde_json::to_string(&cap).expect("serialize");
        let back: Capability = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(cap, back);
    }

    #[test]
    fn allows_matches_same_variant_and_glob() {
        let cap = Capability::FileRead("docs/**".to_string());
        assert!(cap.allows(&Request::FileRead("docs/design.md".to_string())));
        assert!(!cap.allows(&Request::FileRead("src/main.rs".to_string())));
    }

    #[test]
    fn allows_rejects_mismatched_variant_even_with_matching_string() {
        // Same literal string, different Request variant than the grant —
        // must not be allowed just because the strings happen to match.
        let cap = Capability::FileRead("git commit -m x".to_string());
        assert!(!cap.allows(&Request::ShellExec("git commit -m x".to_string())));
    }
}
