use std::process::exit;

mod cli;
mod plugins;

fn main() {
    if let Err(e) = cli::Cli::run() {
        eprintln!("{e}");
        exit(1)
    }
}
