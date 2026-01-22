pub mod cli;
pub mod commands;

use clap::Parser;
use std::fmt;
use std::path::Path;

use cli::{Cli, Commands};
use commands::{RunError, run};

#[derive(Debug)]
pub enum CliError {
    RunError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::RunError(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<RunError> for CliError {
    fn from(err: RunError) -> Self {
        CliError::RunError(err.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            file,
            verbose,
            args,
        } => {
            let file_path = file.as_deref().map(Path::new);
            run::execute(file_path, args, verbose).await?;
        }
    }

    Ok(())
}
