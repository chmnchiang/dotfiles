use clap::{ArgMatches, SubCommand, App};

use git2::Repository;
use Runner;
use ops;
use ops::path::ensure_parent_dir;
use ops::context::Context;

use common::*;

use me::INSTALL_DIR;

const DEIN_URL: &str = "https://github.com/Shougo/dein.vim";

lazy_static! {
    static ref DEIN_PATH: HomePath = HomePath(".vim/dein.vim".into());
    static ref NVIM_INSTALL_DIR: HomePath = HomePath(".config/nvim".into());
    static ref DOTFILES_NVIM_DIR: HomePath = {
        INSTALL_DIR.join("dotfiles/nvim")
    };
}


pub struct Neovim {}

impl Neovim {
    fn install(context: Context) -> Result<()> {

        let logger = &context.logger;
        let dein_path = DEIN_PATH.try_into_path()?;

        ensure_parent_dir(&context, &dein_path)?;

        info!(logger, "cloning repo {} into {}", DEIN_URL, dein_path.to_string_lossy());

        if !context.is_dry_run {
            Repository::clone(DEIN_URL, &dein_path)?;
        }

        let nvim_install_dir = NVIM_INSTALL_DIR.try_into_path()?;
        ensure_parent_dir(&context, &nvim_install_dir)?;

        let dotfiles_nvim_dir = DOTFILES_NVIM_DIR.try_into_path()?;

        ops::path::symlink(&context, &dotfiles_nvim_dir, &nvim_install_dir)?;

        Ok(())
    }
}


impl Neovim {
    fn clean(context: Context) -> Result<()> {
        let dein_path = DEIN_PATH.try_into_path()?;
        ops::path::ensure_clean(&context, dein_path)?;

        let nvim_install_dir = NVIM_INSTALL_DIR.try_into_path()?;
        ops::path::ensure_clean(&context, nvim_install_dir)?;
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
