use crate::util::null_to_default;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Deserialize, Debug, Default)]
pub struct ImageInspect {
    #[serde(rename = "Config")]
    pub config: ImageConfig,

    #[serde(rename = "HostConfig", default, deserialize_with = "null_to_default")]
    pub host_config: ImageHostConfig,
}

#[derive(Deserialize, Debug, Default)]
pub struct ImageConfig {
    #[serde(rename = "Env")]
    pub env: HashSet<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ImageHostConfig {
    #[serde(rename = "Binds", default, deserialize_with = "null_to_default")]
    pub binds: HashSet<String>,

    #[serde(rename = "VolumesFrom", default, deserialize_with = "null_to_default")]
    pub volumes_from: HashSet<String>,

    #[serde(rename = "CapAdd", default, deserialize_with = "null_to_default")]
    pub cap_add: HashSet<String>,

    #[serde(rename = "CapDrop", default, deserialize_with = "null_to_default")]
    pub cap_drop: HashSet<String>,

    #[serde(rename = "Dns")]
    pub dns: HashSet<String>,
}
