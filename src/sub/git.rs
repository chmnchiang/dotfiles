use clap::{ArgMatches, SubCommand, App};

use crate::Runner;
use crate::ops::{
    Op,
    path::SymLink,
};

use crate::common::*;

pub struct Git {}

impl Installable for Git {
    #[allow(non_snake_case)]
    fn generate_op() -> Box<dyn Op> {
        let GIT_CONF_PATH = HomePath(".gitconfig".into());
        let DOTFILES_GIT_CONF_PATH = DotfilesPath("git/gitconfig".into());

        Box::new(
            SymLink {
                src: DOTFILES_GIT_CONF_PATH.into(),
                dst: GIT_CONF_PATH.into(),
            }
        )
    }
}

impl Runner for Git {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("git")
            .about("Setting configuration files of git")
            .subcommand(
                SubCommand::with_name("install")
                    .about("install git configuration")
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
