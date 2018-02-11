use clap::{ArgMatches, SubCommand, App};

use Runner;
use ops;
use slog::Logger;

use common::*;

pub struct Zsh {}

impl Zsh {
    #[cfg(target_os = "linux")]
    fn install() -> Result<()> {

        //let home_dir = ops::path::get_home_dir()?;
        //let zshrc_path = home_dir.join(".zshrc");

        //let dotfiles_zshrc_path = home_dir.join(".config/dotfiles/dotfiles/zsh/zshrc");

        //ops::path::symlink(&dotfiles_zshrc_path, &zshrc_path)?;

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn install() {
        unimplemented!()
    }
}


impl Zsh {
    #[cfg(target_os = "linux")]
    fn clean() -> Result<()> {
        //let home_dir = ops::path::get_home_dir()?;
        //let zshrc_path = home_dir.join(".zshrc");
        //ops::path::ensure_clean(zshrc_path)?;
        Ok(())
    }
}

impl Runner for Zsh {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("zsh")
            .about("Setting configuration files of zsh")
            .subcommand(SubCommand::with_name("install"))
            .subcommand(SubCommand::with_name("clean"))
    }

    fn run(argm: &ArgMatches, logger: Logger) -> Result<()> {
        match argm.subcommand_name() {
            Some(name) => match name {
                "install" => {
                    Self::install()?;
                }
                "clean" => {
                    Self::clean()?;
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
