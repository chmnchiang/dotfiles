use std::env;
use std::fs;
use std::os;
use clap::{ArgMatches, SubCommand, App};

use Runner;
use utils;
use utils::path::ensure_parent_dir;

mod errors {
    use utils;

    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
        links {
            Path(utils::path::Error, utils::path::ErrorKind);
        }
    }
}

use self::errors::*;

pub struct Me {}

impl Me {
    #[cfg(target_os = "linux")]
    fn install() -> Result<()> {
        let home_dir = utils::path::get_home_dir()?;
        let target_dir = home_dir.join(".config/dotfiles");

        ensure_parent_dir(&target_dir)?;

        let cur_dir = env::current_dir()?;

        fs::rename(&cur_dir, &target_dir)?;

        let bin_path = target_dir.join("target/release/dotfiles");
        
        let bin_symlink_path = home_dir.join(".local/bin/dotfl");
        ensure_parent_dir(&bin_symlink_path)?;

        os::unix::fs::symlink(&bin_path, &bin_symlink_path)?;

        Ok(())
    }
}

impl Runner for Me {
    type Error = Error;

    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("self")
            .about("Command about this deployer")
            .subcommand(SubCommand::with_name("install"))
    }

    fn run(argm: &ArgMatches) -> ::std::result::Result<(), Self::Error> {
        match argm.subcommand_name().expect("No subcommand found") {
            "install" => {
                Self::install()?;
            }
            _ => unreachable!(),
        };
        Ok(())
    }
}
