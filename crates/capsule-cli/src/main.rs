pub mod cli;
pub mod commands;

use std::fmt;
use std::path::Path;
use clap::Parser;

use cli::{Cli, Commands};
use commands::{run, RunError};

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
        Commands::Run { file, args } => {
            run::execute(Path::new(&file), args).await?;
        }
    }

    Ok(())
}
