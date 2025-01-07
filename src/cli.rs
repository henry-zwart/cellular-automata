use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub rule: u8,

    #[arg(short, long, default_value_t = 10)]
    pub generations: usize,

    #[arg(short, long, default_value_t = 42)]
    pub seed: u64,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
