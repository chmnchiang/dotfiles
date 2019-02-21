use clap::{ArgMatches, SubCommand, App};

use crate::Runner;
use crate::ops::{
    Op,
    path::SymLink,
};

use crate::common::*;

pub struct Zsh {}

impl Installable for Zsh {
    #[allow(non_snake_case)]
    fn generate_op() -> Box<dyn Op> {
        let ZSHRC_PATH = HomePath(".zshrc".into());
        let DOTFILES_ZSHRC_PATH = DotfilesPath("zsh/zshrc".into());

        Box::new(
            SymLink {
                src: DOTFILES_ZSHRC_PATH.into(),
                dst: ZSHRC_PATH.into(),
            }
        )
    }
}

impl Runner for Zsh {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("zsh")
            .about("Setting configuration files of zsh")
            .subcommand(
                SubCommand::with_name("install")
                    .about("install zsh configuration")
            )
    }

    fn run(argm: &ArgMatches, context: &Context) -> Result<()> {
        match argm.subcommand_name() {
            Some(name) => generate_command!(name, context; install),
            None => {
                Self::build_cli().print_help().unwrap();
                println!();
            }
        };
        Ok(())
    }
}
