use crate::cli::Opts;
use clap::Clap;
use clap::{crate_name, IntoApp};
use clap_generate::generate;
use clap_generate::generators::{Bash, Zsh};
use std::io;

#[derive(Clap, Debug)]
pub struct Completion {
    #[clap(arg_enum)]
    input: Shell,
}

#[derive(Debug, Clap)]
enum Shell {
    Bash,
    Zsh,
}

pub fn print_completions(c: &Completion) {
    let mut app = Opts::into_app();
    match c.input {
        Shell::Bash => generate::<Bash, _>(&mut app, crate_name!(), &mut io::stdout()),
        Shell::Zsh => generate::<Zsh, _>(&mut app, crate_name!(), &mut io::stdout()),
    }
}
