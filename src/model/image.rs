use crate::util::null_to_default;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize, Debug, Default)]
pub struct ImageInspect {
    #[serde(rename = "Config")]
    pub config: ImageConfig,
}

#[derive(Deserialize, Debug, Default)]
pub struct ImageConfig {
    #[serde(rename = "Env")]
    pub env: HashSet<String>,
    #[serde(rename = "Volumes", default, deserialize_with = "null_to_default")]
    pub volumes: HashSet<String>,
}
