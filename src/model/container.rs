use crate::util::{empty_string_is_none, null_to_default};

use serde::Deserialize;
use std::{borrow::Cow, collections::BTreeMap, fmt::Display};

/// A trait for printing attributes of a struct
///
/// Useful when the logic for printing attributes is complex
/// and may not return a value for all cases
pub trait AttrPrinter {
    fn print(&self) -> Option<Cow<str>>;
}

#[derive(Deserialize, Debug)]
pub struct DockerInspect {
    #[serde(rename = "Id")]
    pub id: String,

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

    #[serde(rename = "Memory", default)]
    pub memory: usize,

    #[serde(rename = "MemoryReservation", default)]
    pub memory_reservation: usize,

    #[serde(rename = "ExtraHosts", default, deserialize_with = "null_to_default")]
    pub extra_hosts: Vec<String>,

    #[serde(rename = "Devices", default, deserialize_with = "null_to_default")]
    pub devices: Vec<Device>,

    #[serde(rename = "RestartPolicy")]
    pub restart_policy: RestartPolicy,
}

#[derive(Deserialize, Debug)]
pub struct Device {
    #[serde(rename = "PathOnHost")]
    pub path_on_host: String,

    #[serde(rename = "PathInContainer")]
    pub path_in_container: String,

    #[serde(rename = "CgroupPermissions")]
    pub cgroup_permissions: String,
}

#[derive(Deserialize, Debug)]
pub struct RestartPolicy {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "MaximumRetryCount")]
    pub maximum_retry_count: i32,
}

impl AttrPrinter for RestartPolicy {
    fn print(&self) -> Option<Cow<str>> {
        match self.name.as_str() {
            "always" => Some(Cow::Borrowed("always")),
            "unless-stopped" => Some(Cow::Borrowed("unless-stopped")),
            "on-failure" => Some(Cow::Owned(format!(
                "on-failure:{}",
                self.maximum_retry_count
            ))),
            _ => None,
        }
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cgroup_permissions != "rwm" {
            write!(
                f,
                "{}:{}:{}",
                self.path_on_host, self.path_in_container, self.cgroup_permissions
            )
        } else {
            write!(f, "{}:{}", self.path_on_host, self.path_in_container)
        }
    }
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

    #[serde(rename = "Cmd", default, deserialize_with = "null_to_default")]
    pub cmd: Vec<String>,

    #[serde(rename = "Image")]
    pub image: String,

    #[serde(rename = "WorkingDir", deserialize_with = "empty_string_is_none")]
    pub working_dir: Option<String>,

    #[serde(rename = "MacAddress")]
    pub mac_address: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NetworkSettings {
    #[serde(rename = "MacAddress", deserialize_with = "empty_string_is_none")]
    pub mac_address: Option<String>,

    #[serde(rename = "Ports")]
    pub ports: Ports,
}

impl AttrPrinter for Ports {
    fn print(&self) -> Option<Cow<str>> {
        if self.0.is_empty() {
            return None;
        }

        let mut parts = vec![];
        for (port_proto, options) in &self.0 {
            let (port, protocol) = port_proto.split_once('/')?;
            let protocol = if protocol.eq_ignore_ascii_case("tcp") {
                ""
            } else {
                "/udp"
            };

            if options.is_empty() {
                parts.push(format!("--expose={port}/{protocol}"));
            } else {
                for host in options {
                    let PortInfo { host_port, host_ip } = host;

                    let host_port = host_port.as_deref().filter(|p| !matches!(*p, "0"));
                    let host_ip = host_ip
                        .as_deref()
                        .filter(|ip| !matches!(*ip, "0.0.0.0" | "::"));

                    if let (Some(host_port), Some(host_ip)) = (host_port, host_ip) {
                        parts.push(format!("-p {host_ip}:{host_port}:{port}{protocol}",));
                    }
                }
            }
        }

        Some(Cow::Owned(parts.join(" ")))
    }
}

#[derive(Deserialize, Debug)]
pub struct Ports(BTreeMap<String, Vec<PortInfo>>);

#[derive(Deserialize, Debug)]
pub struct PortInfo {
    #[serde(rename = "HostPort", deserialize_with = "empty_string_is_none")]
    pub host_port: Option<String>,

    #[serde(rename = "HostIp", deserialize_with = "empty_string_is_none")]
    pub host_ip: Option<String>,
}
