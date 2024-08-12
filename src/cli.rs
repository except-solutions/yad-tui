use clap::Parser;

/// YAD - simple Terminal UI for sync yandex disk with local directory
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to config, default .config/yad/Config.toml
    #[arg(short, long, default_value_t = String::from(".config/yad/Config.toml"))]
    pub conf: String,
}

pub fn parse_args() -> Args {
    let args = Args::parse();
    args
}
