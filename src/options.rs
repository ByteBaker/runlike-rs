use clap::Parser;
use std::{
    io::{self, Write},
    process::Command,
};

use crate::{args::CliArgs, model};

/// Options for parsing and printing
#[derive(Debug)]
pub(super) struct Options {
    inspect: model::DockerInspect,
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
            let output = Command::new("docker")
                .args(["inspect", container])
                .output()
                .map_err(|_| "FATAL: docker is not installed")?;

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
                let inspect = arr.remove(0);
                Ok(Self {
                    inspect,
                    pretty,
                    no_name,
                })
            }
            _ => Err("Failed to parse 'docker inspect' output".to_string()),
        }
    }
}

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

impl Options {
    pub(super) fn try_new() -> Result<Self, String> {
        let args = CliArgs::parse();

        Options::try_from(args)
    }
    pub(super) fn print(&self) {
        let inspect = &self.inspect;
        let sep = if self.pretty { " \\\n\t" } else { " " };
        let mut out = io::stdout();

        arg!(out, "docker run ");

        arg!(if !self.no_name; out | sep, "--name=", inspect.name.trim_start_matches('/'));
        arg!(out | sep, "--hostname=", inspect.config.hostname);

        arg!(inspect
            .config
            .mac_address
            .as_deref()
            .or(inspect.network_settings.mac_address.as_deref()); out | sep, "--mac-address=");

        arg!(inspect.host_config.pid_mode; out | sep, "--pid=");
        arg!(inspect.host_config.cpuset_cpus; out | sep, "--cpuset-cpus=");
        arg!(inspect.host_config.cpuset_mems; out | sep, "--cpuset-mems=");

        arg!(if inspect.host_config.network_mode.eq("default"); out | sep, "--network=", inspect.host_config.network_mode);
        arg!(if inspect.host_config.privileged; out | sep, "--privileged");

        arg!(inspect.host_config.runtime; out | sep, "--runtime=");

        arg!(if !inspect.config.attach_stdout; out | sep, "--detach=true");
        arg!(if inspect.host_config.auto_remove; out | sep, "--rm");
        arg!(inspect.config.user; out | sep, "--user=");
        arg!(if inspect.config.tty; out | sep, "-t");
        arg!(inspect.config.working_dir; out | sep, "--workdir=");
        arg!(out | sep, inspect.config.image);

        arg!(inspect.config.cmd.first(); out);

        for c in inspect.config.cmd.iter().skip(1) {
            arg!(out | c, " "); // Because these go together
        }

        writeln!(out).unwrap();
    }
}
