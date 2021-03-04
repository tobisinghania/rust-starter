mod subcommands;

use crate::cli::subcommands::completions::{print_completions, Completion};
use crate::conf;
use crate::conf::init_config;
use clap::Clap;
use clap::{crate_authors, crate_version};
use thiserror::Error;
use tracing::debug;

pub struct Cli;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("{0}")]
    ConfigError(conf::ConfigError),
}

impl From<conf::ConfigError> for CliError {
    fn from(e: conf::ConfigError) -> Self {
        CliError::ConfigError(e)
    }
}

impl Cli {
    pub fn init() -> Result<(), CliError> {
        debug!("Parsing command line arguments");
        let opts: Opts = Opts::parse();

        let config_files = opts.configs.unwrap_or(vec![]);
        let _settings = init_config(config_files)?;

        if let Some(Subcommand::Completion(ref c)) = opts.subcommand {
            print_completions(c);
        }
        debug!("Finished parsing command line arguments");

        Ok(())
    }
}

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!("\n"))]
pub struct Opts {
    /// List of conf files with descending priority. Can be overriden with the env variable CONFIG_PATHS
    #[clap(short, long)]
    configs: Option<Vec<String>>,
    // /// Some input. Because this isn't an Option<T> it's required to be used
    // input: String,
    // /// A level of verbosity, and can be used multiple times
    // #[clap(short, long, parse(from_occurrences))]
    // verbose: i32,
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(Clap, Debug)]
enum Subcommand {
    #[clap(version = crate_version!(), author = crate_authors!("\n"))]
    Completion(Completion),
}
