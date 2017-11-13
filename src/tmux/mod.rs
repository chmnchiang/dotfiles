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

pub struct Tmux {}

impl Tmux {
    #[cfg(target_os = "linux")]
    fn install() -> Result<()> {

        let home_dir = utils::path::get_home_dir()?;
        let tmux_conf_path = home_dir.join(".tmux.conf");

        let dotfiles_tmux_conf_path = home_dir.join(".config/dotfiles/dotfiles/tmux/tmux.conf");

        utils::path::symlink(&dotfiles_tmux_conf_path, &tmux_conf_path)?;

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn install() {
        unimplemented!()
    }
}


impl Tmux {
    #[cfg(target_os = "linux")]
    fn clean() -> Result<()> {
        let home_dir = utils::path::get_home_dir()?;
        let tmux_conf_path = home_dir.join(".tmux.conf");
        utils::path::ensure_clean(tmux_conf_path)?;
        Ok(())
    }
}

impl Runner for Tmux {
    type Error = Error;

    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("tmux")
            .about("Setting configuration files of tmux")
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
