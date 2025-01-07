use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub rule: u8,

    #[arg(short, long, default_value_t = 10)]
    pub generations: usize,

    #[arg(short, long, default_value_t = 42)]
    pub seed: u64,

    #[arg(value_enum, short, long, default_value_t=Initialisation::Middle)]
    pub init: Initialisation,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Initialisation {
    Random,
    Middle,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
