use crate::util::{empty_string_is_none, null_to_default};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DockerInspect {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Config")]
    pub config: Config,

    #[serde(rename = "HostConfig")]
    pub host_config: HostConfig,

    #[serde(rename = "NetworkSettings")]
    pub network_settings: NetworkSettings,
}

#[derive(Deserialize, Debug)]
pub struct HostConfig {
    #[serde(rename = "NetworkMode")]
    pub network_mode: String,

    #[serde(rename = "Runtime")]
    pub runtime: Option<String>,

    #[serde(rename = "CpusetCpus", deserialize_with = "empty_string_is_none")]
    pub cpuset_cpus: Option<String>,

    #[serde(rename = "CpusetMems", deserialize_with = "empty_string_is_none")]
    pub cpuset_mems: Option<String>,

    #[serde(rename = "AutoRemove")]
    pub auto_remove: bool,

    #[serde(rename = "PidMode", deserialize_with = "empty_string_is_none")]
    pub pid_mode: Option<String>,

    #[serde(rename = "Privileged")]
    pub privileged: bool,

    #[serde(rename = "Binds", default, deserialize_with = "null_to_default")]
    pub binds: Vec<String>,

    #[serde(rename = "VolumesFrom", default, deserialize_with = "null_to_default")]
    pub volumes_from: Vec<String>,

    #[serde(rename = "CapAdd", default, deserialize_with = "null_to_default")]
    pub cap_add: Vec<String>,

    #[serde(rename = "CapDrop", default, deserialize_with = "null_to_default")]
    pub cap_drop: Vec<String>,

    #[serde(rename = "Dns", default, deserialize_with = "null_to_default")]
    pub dns: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "Hostname")]
    pub hostname: String,

    #[serde(rename = "Env")]
    pub env: Vec<String>,

    #[serde(rename = "AttachStdout")]
    pub attach_stdout: bool,

    #[serde(rename = "Tty")]
    pub tty: bool,

    #[serde(rename = "User", deserialize_with = "empty_string_is_none")]
    pub user: Option<String>,

    #[serde(rename = "Cmd")]
    pub cmd: Vec<String>,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "WorkingDir", deserialize_with = "empty_string_is_none")]
    pub working_dir: Option<String>,

    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,

    #[serde(rename = "Volumes", default, deserialize_with = "null_to_default")]
    pub volumes: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct NetworkSettings {
    #[serde(rename = "MacAddress", deserialize_with = "empty_string_is_none")]
    pub mac_address: Option<String>,
}
