use std::env;
use std::path::PathBuf;
use std::fs;
use std::os;
use clap::{ArgMatches, SubCommand, App};

use Runner;
use utils::*;

mod error {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

pub struct Me {}

impl Me {
    #[cfg(target_os = "linux")]
    fn install() -> error::Result<()> {
        let home_var = get_home_var();
        let config_dir: PathBuf = [&home_var, ".config"].iter().collect();
        ensure_dir(&config_dir)?;
        let mut target_dir = config_dir;
        target_dir.push("dotfiles");
        let cur_dir = env::current_dir()?;

        fs::rename(&cur_dir, &target_dir)?;

        let bin_dir = {
            let mut dir = target_dir;
            dir.push("target/release/dotfiles");
            dir
        };
        let bin_symlink_dir: PathBuf = [&home_var, ".local", "bin", "dotfl"].iter().collect();
        os::unix::fs::symlink(&bin_dir, &bin_symlink_dir)?;

        Ok(())
    }
}

impl Runner for Me {
    type Error = error::Error;

    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("self")
            .about("Setting configuration files of neovim")
            .subcommand(SubCommand::with_name("install"))
    }

    fn run(argm: &ArgMatches) -> Result<(), Self::Error> {
        match argm.subcommand_name().expect("No subcommand found") {
            "install" => { 
                Self::install()?;
            },
            _ => unreachable!()
        };
        Ok(())
    }
}
