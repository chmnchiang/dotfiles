use clap::{ArgMatches, SubCommand, App};

use Runner;
use utils;

mod errors {
    use utils;

    error_chain!{
        links {
            Path(utils::path::Error, utils::path::ErrorKind);
        }
    }
}

use self::errors::*;

pub struct Zsh {}

impl Zsh {
    #[cfg(target_os = "linux")]
    fn install() -> Result<()> {

        let home_dir = utils::path::get_home_dir()?;
        let zshrc_path = home_dir.join(".zshrc");

        let dotfiles_zshrc_path = home_dir.join(".config/dotfiles/dotfiles/zsh/zshrc");

        utils::path::symlink(&dotfiles_zshrc_path, &zshrc_path)?;

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
        let home_dir = utils::path::get_home_dir()?;
        let zshrc_path = home_dir.join(".zshrc");
        utils::path::ensure_clean(zshrc_path)?;
        Ok(())
    }
}

impl Runner for Zsh {
    type Error = Error;

    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("zsh")
            .about("Setting configuration files of zsh")
            .subcommand(SubCommand::with_name("install"))
            .subcommand(SubCommand::with_name("clean"))
    }

    fn run(argm: &ArgMatches) -> ::std::result::Result<(), Self::Error> {
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
            }
        };
        Ok(())
    }
}
