use clap::{ArgMatches, SubCommand, App};

use git2::Repository;
use Runner;
use ops;
use ops::path::ensure_parent_dir;
use ops::context::Context;
use slog::Logger;

use common::*;

const DEIN_URL: &str = "https://github.com/Shougo/dein.vim";

pub struct Neovim {}

impl Neovim {
    #[cfg(target_os = "linux")]
    fn install(context: Context) -> Result<()> {
        //let logger = &context.logger;

        //let home_dir = ops::path::get_home_dir()?;

        //let dein_path = home_dir.join(".vim/dein.vim");

        //ensure_parent_dir(&dein_path)?;

        //info!(logger, "cloning {} to {}", DEIN_URL, dein_path.to_string_lossy());
        //Repository::clone(DEIN_URL, &dein_path)?;

        //let nvim_config_dir = home_dir.join(".config/nvim");
        //ensure_parent_dir(&nvim_config_dir)?;

        //let dotfiles_nvim_dir = home_dir.join(".config/dotfiles/dotfiles/nvim");

        //ops::path::symlink(&dotfiles_nvim_dir, &nvim_config_dir)?;

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn install() {
        unimplemented!()
    }
}


impl Neovim {
    #[cfg(target_os = "linux")]
    fn clean(context: Context) -> Result<()> {
        //let home_dir = ops::path::get_home_dir()?;
        //let dein_dir = home_dir.join(".vim/dein.vim");
        //ops::path::ensure_clean(dein_dir)?;

        //let nvim_dir = home_dir.join(".config/nvim");
        //ops::path::ensure_clean(nvim_dir)?;
        Ok(())
    }
}

impl Runner for Neovim {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("neovim")
            .about("Setting configuration files of neovim")
            .subcommand(
                SubCommand::with_name("install")
                    .about("install neovim configuration")
            )
            .subcommand(
                SubCommand::with_name("clean")
                    .about("cleaning neovim configuration files")
            )
    }

    fn run(argm: &ArgMatches, logger: Logger) -> Result<()> {
        let context = Context {
            logger: logger,
            is_dry_run: argm.is_present("dry"),
        };

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
