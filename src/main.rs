use tracing_subscriber;

use sample_app::cli::Cli;
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
