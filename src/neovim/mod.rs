use std::env;
use std::path::{Path, PathBuf};
use std::fs;
use std::os;
use clap::{ArgMatches, SubCommand, App};

use git2::Repository;
use Runner;
use utils::*;
use slog_scope;

mod error {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Git(::git2::Error);
        }
    }
}

const DEIN_URL: &'static str = "https://github.com/Shougo/dein.vim";
pub struct Neovim { }

impl Neovim {
    #[cfg(target_os = "linux")]
    fn install() -> error::Result<()> {
        let logger = slog_scope::logger();

        let home_dir = Path::new(&get_home_var()).to_owned();
        
        let mut vim_dir = home_dir.clone();
        vim_dir.push(".vim");

        // Ensure .vim/
        ensure_dir(&vim_dir)?;

        // Clone dein.vim
        let mut dein_dir = vim_dir;
        dein_dir.push("dein.vim");
        Repository::clone(DEIN_URL, &dein_dir)?;

        // Make nvim config dir
        let mut config_dir = home_dir.clone();
        config_dir.push(".config");

        ensure_dir(&config_dir)?;

        let mut nvim_dir = config_dir;
        nvim_dir.push("nvim");

        let mut dotfiles_nvim_dir = env::current_dir()?;
        dotfiles_nvim_dir.push("dotfiles");
        dotfiles_nvim_dir.push("nvim");

        os::unix::fs::symlink(&dotfiles_nvim_dir, &nvim_dir)
            .expect(&format!("Create symlink {} -> {} failed",
                            dotfiles_nvim_dir.to_string_lossy(),
                            nvim_dir.to_string_lossy()));

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    fn install() {
        unimplemented!()
    }
}


impl Neovim {
    #[cfg(target_os = "linux")]
    fn clean() -> error::Result<()> {
        let home_var = get_home_var();
        let dein_dir: PathBuf = [&home_var, ".vim", "dein.vim"].iter().collect();
        fs::remove_dir_all(dein_dir)?;
        let nvim_dir: PathBuf = [&home_var, ".config", "nvim"].iter().collect();
        fs::remove_file(nvim_dir)?;
        Ok(())
    }
}

impl Runner for Neovim {
    type Error = error::Error;

    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("neovim")
            .about("Setting configuration files of neovim")
            .subcommand(SubCommand::with_name("install"))
            .subcommand(SubCommand::with_name("clean"))
    }

    fn run(argm: &ArgMatches) -> Result<(), Self::Error> {

        match argm.subcommand_name().expect("No subcommand found") {
            "install" => { 
                Self::install()?;
            },
            "clean" => { 
                Self::clean()?;
            },
            _ => unreachable!()
        };
        Ok(())
    }
}
