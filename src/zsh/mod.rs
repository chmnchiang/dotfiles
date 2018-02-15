use clap::{ArgMatches, SubCommand, App};

use Runner;
use ops;

use common::*;

pub struct Zsh {}

use me::INSTALL_DIR;

lazy_static! {
    static ref ZSHRC_PATH: HomePath = HomePath(".zshrc".into());
    static ref DOTFILES_ZSHRC_PATH: HomePath = {
        INSTALL_DIR.join("dotfiles/zsh/zshrc")
    };
}

impl Zsh {
    fn install(context: Context) -> Result<()> {

        let zshrc_path = ZSHRC_PATH.try_into_path()?;
        let dotfiles_zshrc_path = DOTFILES_ZSHRC_PATH.try_into_path()?;
        ops::path::symlink(&context, &dotfiles_zshrc_path, &zshrc_path)?;

        Ok(())
    }
}


impl Zsh {
    fn clean(context: Context) -> Result<()> {
        let zshrc_path = ZSHRC_PATH.try_into_path()?;
        ops::path::ensure_clean(&context, zshrc_path)?;
        Ok(())
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
            .subcommand(
                SubCommand::with_name("clean")
                    .about("cleaning zsh configuration files")
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
