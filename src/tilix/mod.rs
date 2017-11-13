use clap::{ArgMatches, SubCommand, App};
use std::process::Command;
use std::fs::OpenOptions;

use Runner;
use utils;
use slog_scope;

mod errors {
    use utils;

    error_chain!{
        links {
            Path(utils::path::Error, utils::path::ErrorKind);
        }
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

use self::errors::*;

pub struct Tilix {}

impl Tilix {
    #[cfg(target_os = "linux")]
    fn install() -> Result<()> {
        let logger = slog_scope::logger();

        let home_dir = utils::path::get_home_dir()?;
        let dconf_path = home_dir.join(".config/dotfiles/dotfiles/tilix/tilix.dconf");
        info!(logger, "load tilix dconf file from {}", dconf_path.to_string_lossy());
        let file = OpenOptions::new()
            .read(true)
            .open(&dconf_path)?;

        Command::new("dconf")
            .arg("load")
            .arg("/com/gexperts/Tilix/")
            .stdin(file)
            .spawn()?;
            
        Ok(())
    }
}


impl Tilix {
    #[cfg(target_os = "linux")]
    fn save() -> Result<()> {
        let logger = slog_scope::logger();

        let home_dir = utils::path::get_home_dir()?;
        let dconf_path = home_dir.join(".config/dotfiles/dotfiles/tilix/tilix.dconf");
        info!(logger, "dump tilix dconf file into {}", dconf_path.to_string_lossy());
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&dconf_path)?;

        Command::new("dconf")
            .arg("dump")
            .arg("/com/gexperts/Tilix/")
            .stdout(file)
            .spawn()?;
            

        Ok(())
    }
}

impl Runner for Tilix {
    type Error = Error;

    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("tilix")
            .about("Setting configuration files of tmux")
            .subcommand(SubCommand::with_name("install"))
            .subcommand(SubCommand::with_name("save"))
    }

    fn run(argm: &ArgMatches) -> ::std::result::Result<(), Self::Error> {
        match argm.subcommand_name() {
            Some(name) => match name {
                "install" => {
                    Self::install()?;
                }
                "save" => {
                    Self::save()?;
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
