use clap::{ArgMatches, SubCommand, App};

use crate::Runner;
use crate::ops::{
    Op,
    path::SymLink,
};

use crate::common::*;

pub struct Tmux {}

impl Installable for Tmux {
    #[allow(non_snake_case)]
    fn generate_op() -> Box<dyn Op> {
        let TMUX_CONF_PATH = HomePath(".tmux.conf".into());
        let DOTFILES_TMUX_CONF_PATH = DotfilesPath("tmux/tmux.conf".into());

        Box::new(
            SymLink {
                src: DOTFILES_TMUX_CONF_PATH.into(),
                dst: TMUX_CONF_PATH.into(),
            }
        )
    }
}


impl Runner for Tmux {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("tmux")
            .about("Setting configuration files of tmux")
            .subcommand(
                SubCommand::with_name("install")
                    .about("install tmux configuration")
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
