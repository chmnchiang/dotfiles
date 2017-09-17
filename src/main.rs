extern crate clap;
extern crate git2;

use clap::{App};
mod neovim;
use neovim::Neovim;

trait Installer {
    fn build_cli() -> App<'static, 'static>;
    fn run();
}

fn build_cli() -> App<'static, 'static> {
    App::new("Dotfiles")
        .version("1.0")
        .subcommand(Neovim::build_cli())
}

fn main() {

    let app = build_cli();
    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("neovim") => { 
            println!("Running neovim config installer...");
            neovim::Neovim::run();
        },
        None => { build_cli().print_help().unwrap(); },
        _ => unreachable!(),
    };
}
