use std::{
    env,
    fs,
    os,
    path::Path,
};
use clap::{ArgMatches, SubCommand, App};

use Runner;
use ops;
use ops::path::ensure_parent_dir;
use slog::Logger;

use common::*;

pub struct Me {}

lazy_static! {
    static ref INSTALL_DIR: HomePath = HomePath(".config/dotfiles".into());
    static ref BIN_PATH: HomePath = {
        INSTALL_DIR.join(Path::new("target/release/dotfiles"))
    };
}

impl Me {
    #[cfg(target_os = "linux")]
    fn install() -> Result<()> {
        //let install_dir = INSTALL_DIR.try_into_path()?;

        //ensure_parent_dir(&target_dir)?;

        //let cur_dir = env::current_dir()?;

        //fs::rename(&cur_dir, &target_dir)?;

        //let bin_path = target_dir.join("target/release/dotfiles");
        
        //let bin_symlink_path = home_dir.join(".local/bin/dotfl");
        //ensure_parent_dir(&bin_symlink_path)?;

        //os::unix::fs::symlink(&bin_path, &bin_symlink_path)?;

        Ok(())
    }
}

impl Runner for Me {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("self")
            .about("Command about this deployer")
            .subcommand(
                SubCommand::with_name("install")
                    .about("Install this deployer"),
            )
            .subcommand(
                SubCommand::with_name("commit")
                    .about("Save changes and commit"),
            )
    }

    fn run(argm: &ArgMatches, logger: Logger) -> Result<()> {
        match argm.subcommand_name().expect("No subcommand found") {
            "install" => {
                Self::install()?;
            }
            _ => unreachable!(),
        };
        Ok(())
    }
}
