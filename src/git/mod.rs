use clap::{ArgMatches, SubCommand, App};

use Runner;
use ops;

use common::*;

use me::INSTALL_DIR;

pub struct Git {}

lazy_static! {
    static ref GIT_CONF_PATH: HomePath = HomePath(".gitconfig".into());
    static ref DOTFILES_GIT_CONF_PATH: HomePath = {
        INSTALL_DIR.join("dotfiles/git/gitconfig")
    };
}

impl Git {
    fn install(context: Context) -> Result<()> {

        let conf_path = GIT_CONF_PATH.try_into_path()?;
        let dotfiles_conf_path = DOTFILES_GIT_CONF_PATH.try_into_path()?;

        ops::path::symlink(&context, &dotfiles_conf_path, &conf_path)?;

        Ok(())
    }
}


impl Git {
    fn clean(context: Context) -> Result<()> {
        let conf_path = GIT_CONF_PATH.try_into_path()?;
        ops::path::ensure_clean(&context, conf_path)?;
        Ok(())
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
            .subcommand(
                SubCommand::with_name("clean")
                    .about("cleaning git configuration files")
            )
    }

    fn run(argm: &ArgMatches, context: Context) -> Result<()> {
        match argm.subcommand_name() {
            Some(name) => match name {
                "install" => {
                    Self::install(context)?;
                }
                "clean" => {
                    Self::clean(context)?;
                }
                _ => unreachable!(),
            }
            None => {
                Self::build_cli().print_help().unwrap();
                println!();
            }
        };
        Ok(())
    }
}
