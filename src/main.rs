#![feature(trace_macros)]
#![feature(cell_update)]
#![recursion_limit = "1024"]

//trace_macros!(true);

use slog::{Logger, Drain, o};
use slog_scope::GlobalLoggerGuard;
use std::sync::Arc;
use clap::{App, Arg, ArgMatches};

#[macro_use]
mod macro_utils;
//use crate::macro_utils::warn;

mod common;
use crate::common::*;

mod prompt;

//macro_rules! call_with_all_mod {
    //($macro_name:ident) => {
        //$macro_name!(
            //neovim/Neovim
            ////tmux/Tmux
            ////zsh/Zsh
            ////tilix/Tilix
            ////git/Git
        //);
    //};
//}

mod me;
use crate::me::Me;

mod sub;

macro_rules! include_all_mod {
    ($($name:ident/$Name:ident) * ) => {
        $(
            use crate::sub::$name::$Name;
        )*
    };
}

call_with_all_mod!(include_all_mod);

mod runner;
mod ops;
mod common_path;

fn build_cli() -> App<'static, 'static> {
    let app = App::new("Dotfiles")
        .version("1.0")
        .subcommand(Me::build_cli())
        .arg(Arg::with_name("dry")
                .short("n")
                .long("dry-run")
                .global(true)
                .help("preform dry run instead"));

    macro_rules! append_all_mod_on_app {
        ($($name:ident/$Name:ident) *) => {
            |x: App<'static, 'static>| x
                $(.subcommand($Name::build_cli()))*
                //append_all_mod_on_app!($expr.subcommand($Name::build_cli()), $($other/$Other)*);
        };
    }

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

fn main() -> Result<()> {

    let _logger_guard = setup_slog();

    let app = build_cli();
    let matches = app.get_matches();

    macro_rules! generate_match_all_mod {
        ($($name:ident/$Name:ident) * ) => {
            |x| match x {
                $(
                    stringify!($name) => $Name::run2,
                )*
                _ => unreachable!()
            }
        };
    }

    match matches.subcommand_name() {
        Some(subc) => {
            let subm = matches.subcommand_matches(subc).unwrap();

            let func: fn(&ArgMatches) -> Result<()> = match subc {
                "self" => Me::run2,
                other => call_with_all_mod!(generate_match_all_mod)(other),
            };
            func(subm)?;
        }
        None => {
            build_cli().print_help().unwrap();
            println!()
        }
    };

    Ok(())
}
