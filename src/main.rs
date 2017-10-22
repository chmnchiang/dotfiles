#![recursion_limit = "1024"]
extern crate clap;
extern crate git2;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_scope;
extern crate slog_async;

#[macro_use]
extern crate error_chain;

mod utils;
mod me;
mod neovim;

use slog::Drain;
use slog_scope::GlobalLoggerGuard;
use std::sync::Arc;
use clap::{App, ArgMatches};
use me::Me;
use neovim::Neovim;

trait Runner {
    type Error: std::error::Error;

    fn build_cli() -> App<'static, 'static>;
    fn run(&ArgMatches) -> Result<(), Self::Error>;

    fn run_unwrap(argm: &ArgMatches) {
        let logger = slog_scope::logger();
        Self::run(argm).map_err(|error| {
            error!(logger, "{}", error);
        }).unwrap();
    }
}

fn build_cli() -> App<'static, 'static> {
    App::new("Dotfiles")
        .version("1.0")
        .subcommand(Neovim::build_cli())
        .subcommand(Me::build_cli())
}

fn setup_slog() -> GlobalLoggerGuard {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(Arc::new(drain), o!());
    slog_scope::set_global_logger(log)
}

fn main() {

    let _logger_guard = setup_slog();

    let app = build_cli();
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some(subc) => { 
            println!("Running neovim config installer...");
            let subm = matches.subcommand_matches(subc).unwrap();

            let func: fn(&ArgMatches) = match subc {
                "self" => me::Me::run_unwrap,
                "neovim" => neovim::Neovim::run_unwrap,
                _ => unreachable!(),
            };
            func(subm);
        },
        None => { build_cli().print_help().unwrap(); },
    };
}
