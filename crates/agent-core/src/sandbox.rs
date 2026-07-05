//! `SandboxConfig`: wasmtime fuel/epoch resource limits applied uniformly to
//! every agent class (chassis design §7) — fuel counts instructions
//! executed regardless of host-import count, so even the pure-compute
//! classes (planner/generator/test-writer) get one of these.

/// Resource limits applied to a single agent's wasmtime execution (chassis
/// design §7). Uniform shape across all 8 agent classes; per-class *values*
/// are a separate, still-open empirical question (design §7) not decided
/// here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SandboxConfig {
    /// CPU-instruction budget; 0 = unlimited.
    pub fuel_limit: u64,
    /// Wall-clock, epoch-based interruption.
    pub timeout_secs: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_config_has_fuel_limit_and_timeout_fields() {
        let cfg = SandboxConfig {
            fuel_limit: 1_000_000,
            timeout_secs: Some(30),
        };
        assert_eq!(cfg.fuel_limit, 1_000_000);
        assert_eq!(cfg.timeout_secs, Some(30));
    }

    #[test]
    fn sandbox_config_timeout_is_optional() {
        let cfg = SandboxConfig {
            fuel_limit: 0,
            timeout_secs: None,
        };
        assert_eq!(cfg.timeout_secs, None);
    }
}
