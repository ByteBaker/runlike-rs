use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    version,
    about,
    long_about = "This is a Rust implementation of `runlike` written in Python"
)]
pub(super) struct CliArgs {
    /// The container name or ID
    pub container: Option<String>,
    /// Break down the output into multiple lines
    #[arg(short, long)]
    pub pretty: bool,
    #[arg(short, long)]
    pub stdin: bool,
    /// Do not include container name in output
    #[arg(long)]
    pub no_name: bool,
}
