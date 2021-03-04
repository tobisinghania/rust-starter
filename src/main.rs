// (Full example with detailed comments in examples/01d_quick_example.rs)
//
// This example demonstrates clap's full 'custom derive' style of creating arguments which is the
// simplest method of use, but sacrifices some flexibility.
use tracing_subscriber;

use sample_app::cli::{Cli, CliError};
use std::process::exit;
use tracing::error;

fn main() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    let result = Cli::init();

    let return_code = match result {
        Ok(_) => 0,
        Err(error) => {
            error!("{}", error);
            -1
        }
    };

    exit(return_code);
}
