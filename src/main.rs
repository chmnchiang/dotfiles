#![feature(trace_macros)]
#![recursion_limit = "1024"]

//trace_macros!(true);

#[macro_use] extern crate lazy_static;

extern crate clap;
extern crate git2;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_scope;
extern crate slog_async;

#[macro_use]
extern crate failure;

#[macro_use]
mod macro_utils;

macro_rules! call_with_all_mod {
    ($macro_name:ident) => {
        //$macro_name!(neovim/Neovim tmux/Tmux zsh/Zsh tilix/Tilix);
        $macro_name!();
    };
}

mod me;
use me::Me;

call_with_all_mod!(include_all_mod);

mod ops;
mod common;
mod home_path;

use common::*;

use slog::{Logger, Drain};
use slog_scope::GlobalLoggerGuard;
use std::sync::Arc;
use clap::{App, Arg, ArgMatches};

use ops::context::Context;

trait Runner {
    fn build_cli() -> App<'static, 'static>;
    fn run(&ArgMatches, Context) -> Result<()>;

    fn run_unwrap(argm: &ArgMatches) {
        let logger = slog_scope::logger();

        let context = Context {
            logger: logger.clone(),
            is_dry_run: argm.is_present("dry"),
        };

        Self::run(argm, context)
            .map_err(|error| {
                error!(logger, "{}", error);
            })
            .unwrap();
    }
}

fn build_cli() -> App<'static, 'static> {
    let app = App::new("Dotfiles")
        .version("1.0")
        .subcommand(Me::build_cli())
        .arg(Arg::with_name("dry")
                .short("n")
                .long("dry-run")
                .global(true)
                .help("preform dry run instead"));

    call_with_all_mod!(
        append_all_mod_on_app
    )(app)
}

fn setup_slog() -> GlobalLoggerGuard {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(Arc::new(drain), o!());
    slog_scope::set_global_logger(log)
}

fn main() {

    let _logger_guard = setup_slog();

    let app = build_cli();
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some(subc) => {
            let subm = matches.subcommand_matches(subc).unwrap();

            let func: fn(&ArgMatches) = match subc {
                "self" => Me::run_unwrap,
                other => call_with_all_mod!(generate_match_all_mod)(other),
            };
            func(subm);
        }
        None => {
            build_cli().print_help().unwrap();
            println!()
        }
    };
}
