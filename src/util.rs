use std::process::{Command, Output};

use serde::{Deserialize, Deserializer};

/// When deserializing, treat an empty string as `None`.
pub fn empty_string_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok((!s.is_empty()).then_some(s))
}

// Generic custom deserializer that treats `null` as the type's default.
pub(super) fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

/// Run `docker inspect` command
pub(super) fn run_docker_inspect(cmd: &str, arg: &str) -> Result<Output, &'static str> {
    Command::new("docker")
        .args([cmd, "inspect", arg])
        .output()
        .map_err(|_| "FATAL: docker is not installed")
}
