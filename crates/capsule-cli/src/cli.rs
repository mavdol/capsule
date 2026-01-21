use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "capsule")]
#[command(version)]
#[command(about = "A secure, durable runtime for AI agents", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        file: Option<String>,

        #[arg(short, long)]
        verbose: bool,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}
