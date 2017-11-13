use clap::{ArgMatches, SubCommand, App};

use git2::Repository;
use Runner;
use utils;
use utils::path::ensure_parent_dir;
use slog_scope;

mod errors {
    use utils;

    error_chain!{
        links {
            Path(utils::path::Error, utils::path::ErrorKind);
        }
        foreign_links {
            Io(::std::io::Error);
            Git(::git2::Error);
        }
    }
}

use self::errors::*;

const DEIN_URL: &str = "https://github.com/Shougo/dein.vim";

pub struct Neovim {}

impl Neovim {
    #[cfg(target_os = "linux")]
    fn install() -> Result<()> {
        let logger = slog_scope::logger();

        let home_dir = utils::path::get_home_dir()?;

        let dein_path = home_dir.join(".vim/dein.vim");

        ensure_parent_dir(&dein_path)?;

        info!(logger, "cloning {} to {}", DEIN_URL, dein_path.to_string_lossy());
        Repository::clone(DEIN_URL, &dein_path)?;

        let nvim_config_dir = home_dir.join(".config/nvim");
        ensure_parent_dir(&nvim_config_dir)?;

        let dotfiles_nvim_dir = home_dir.join(".config/dotfiles/dotfiles/nvim");

        utils::path::symlink(&dotfiles_nvim_dir, &nvim_config_dir)?;

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn install() {
        unimplemented!()
    }
}


impl Neovim {
    #[cfg(target_os = "linux")]
    fn clean() -> Result<()> {
        let home_dir = utils::path::get_home_dir()?;
        let dein_dir = home_dir.join(".vim/dein.vim");
        utils::path::ensure_clean(dein_dir)?;

        let nvim_dir = home_dir.join(".config/nvim");
        utils::path::ensure_clean(nvim_dir)?;
        Ok(())
    }
}

impl Runner for Neovim {
    type Error = Error;

    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("neovim")
            .about("Setting configuration files of neovim")
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
