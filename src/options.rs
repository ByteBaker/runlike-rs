use clap::Parser;
use std::io::{self, Write};

use crate::{
    args::CliArgs,
    model::{self, AttrPrinter, DockerInspect, ImageInspect},
    util::run_docker_inspect,
};

/// Options for parsing and printing
#[derive(Debug)]
pub(super) struct Options {
    container: DockerInspect,
    image: ImageInspect,
    pretty: bool,
    no_name: bool,
}

impl TryFrom<CliArgs> for Options {
    type Error = String;

    fn try_from(args: CliArgs) -> Result<Self, Self::Error> {
        let CliArgs {
            container,
            no_name,
            pretty,
            stdin,
        } = args;

        let mut stdout = vec![];
        let mut stderr = vec![];
        if let Some(container) = container.as_deref() {
            let output = run_docker_inspect("container", container)?;

            stdout = output.stdout;
            stderr = output.stderr;
        } else if stdin {
            let mut buf = String::new();

            while io::stdin().read_line(&mut buf).is_ok() {
                if buf.trim().is_empty() {
                    break;
                }

                stdout.extend(buf.as_bytes());
                buf.clear();
            }
        }

        if !stderr.is_empty() {
            Err(String::from_utf8_lossy(stderr.as_slice())
                .trim_start_matches("Error:")
                .trim())?;
        }

        if stdout.is_empty() {
            Err("No output from docker inspect")?;
        }

        match serde_json::from_slice::<Vec<model::DockerInspect>>(&stdout) {
            Ok(mut arr) if !arr.is_empty() => {
                let container = arr.remove(0);

                let image = if !stdin {
                    let stdout = run_docker_inspect("image", &container.config.image)?.stdout;
                    serde_json::from_slice::<Vec<model::ImageInspect>>(&stdout)
                        .map_err(|e| {
                            format!("Failed to parse 'docker image inspect' output: {e:?}")
                        })?
                        .remove(0)
                } else {
                    model::ImageInspect::default()
                };

                Ok(Self {
                    container,
                    image,
                    pretty,
                    no_name,
                })
            }
            e => Err(format!("Failed to parse 'docker inspect' output: {e:?}")),
        }
    }
}

/// Writes arguments to the output
macro_rules! arg {
    // Matches a boolean before writing to the output
    (if $eval: expr ; $cmd: ident $(|$delim: ident)? $(, $parts: expr)+) => {
        if $eval {
            arg!($cmd $(|$delim)? $(, $parts)+);
        }
    };

    // Matches an Option before writing to the output
    ($eval: expr ; $cmd: ident $(|$delim: ident)? $(, $part: expr)?) => {
        if let Some(v) = $eval.as_deref() {
            arg!($cmd $(|$delim)? $(, $part)?, v);
        }
    };

    // Just write the output, and delimiter (if any)
    ($cmd: ident $(|$delim: ident)? $(, $parts: expr)+) => {
        $(write!(&mut $cmd, "{}", $parts).unwrap();)+
        $(write!(&mut $cmd, "{}", $delim).unwrap();)?
    };
}

/// Writes multiple arguments to the output, excluding any that are in the image config
macro_rules! multi_arg_excl_image {
    ($cmd: ident | $delim: ident | $opt: expr => $src: expr $(, $image_cfg: expr)?) => {{
        for field in $src.iter()$(.filter(|mnt| !$image_cfg.contains(*mnt)))? {
            arg!($cmd | $delim, $opt, '"', field, '"');
        }
    }};
}

impl Options {
    pub(super) fn try_new() -> Result<Self, String> {
        let args = CliArgs::parse();

        Self::try_from(args)
    }

    pub(super) fn print(self) {
        let inspect = &self.container;
        let image = self.image;

        let sep = if self.pretty { " \\\n\t" } else { " " };
        let mut out = io::stdout();

        arg!(out, "docker run ");

        arg!(if !self.no_name; out | sep, "--name=", inspect.name.trim_start_matches('/'));
        arg!(if !inspect.id.starts_with(&inspect.config.hostname); out | sep, "--hostname=", inspect.config.hostname);
        arg!(inspect.config.user; out | sep, "--user=");

        arg!(inspect
            .config
            .mac_address
            .as_deref()
            .or(inspect.network_settings.mac_address.as_deref()); out | sep, "--mac-address=");

        arg!(inspect.host_config.pid_mode; out | sep, "--pid=");
        arg!(inspect.host_config.cpuset_cpus; out | sep, "--cpuset-cpus=");
        arg!(inspect.host_config.cpuset_mems; out | sep, "--cpuset-mems=");

        multi_arg_excl_image!(out | sep | "--env=" => inspect.config.env, image.config.env);
        multi_arg_excl_image!(out | sep | "--volume=" => inspect.host_config.binds, image.host_config.binds);
        multi_arg_excl_image!(out | sep | "--volumes-from=" => inspect.host_config.volumes_from, image.host_config.volumes_from);
        multi_arg_excl_image!(out | sep | "--cap-add=" => inspect.host_config.cap_add, image.host_config.cap_add);
        multi_arg_excl_image!(out | sep | "--cap-drop=" => inspect.host_config.cap_drop, image.host_config.cap_drop);
        multi_arg_excl_image!(out | sep | "--dns=" => inspect.host_config.dns, image.host_config.dns);

        arg!(if !matches!(inspect.host_config.network_mode.as_str(), "default" | "bridge"); out | sep, "--network=", inspect.host_config.network_mode);
        arg!(if inspect.host_config.privileged; out | sep, "--privileged");

        arg!(inspect.config.working_dir; out | sep, "--workdir=");

        arg!(inspect.host_config.restart_policy.print(); out | sep, "--restart=");

        for device in inspect.host_config.devices.iter() {
            arg!(out | sep, "--device ", device);
        }

        arg!(inspect.network_settings.ports.print(); out | sep);

        for host in inspect.host_config.extra_hosts.iter() {
            arg!(out | sep, "--add-host", " ", host);
        }

        arg!(inspect.host_config.runtime; out | sep, "--runtime=");

        arg!(if !inspect.host_config.memory == 0; out | sep, "--memory=", '"', inspect.host_config.memory, '"');
        arg!(if !inspect.host_config.memory_reservation == 0; out | sep, "--memory-reservation=", '"', inspect.host_config.memory_reservation, '"');

        arg!(if !inspect.config.attach_stdout; out | sep, "--detach=true");
        arg!(if inspect.config.tty; out | sep, "-t");
        arg!(if inspect.host_config.auto_remove; out | sep, "--rm");

        arg!(out | sep, inspect.config.image);

        arg!(inspect.config.cmd.first(); out);

        for c in inspect.config.cmd.iter().skip(1) {
            arg!(out | c, " "); // Because these go together
        }

        writeln!(out).unwrap();
    }
}
