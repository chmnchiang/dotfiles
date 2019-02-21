use clap::{ArgMatches, SubCommand, App};
use std::process::Command;
use std::fs::OpenOptions;

use crate::Runner;

use crate::common::*;

use crate::me::INSTALL_DIR;

lazy_static! {
    static ref DOTFILES_TILIX_DCONF_PATH: HomePath = {
        INSTALL_DIR.join("dotfiles/tilix/tilix.dconf")
    };
}

pub struct Tilix {}

//impl Installable for Tilix {
    //#[allow(non_snake_case)]
    //fn generate_op() -> Box<dyn Op> {
        //let ZSHRC_PATH = HomePath(".zshrc".into());
        //let DOTFILES_ZSHRC_PATH = DotfilesPath("zsh/zshrc".into());

        //Box::new(
            //SymLink {
                //src: DOTFILES_ZSHRC_PATH.into(),
                //dst: ZSHRC_PATH.into(),
            //}
        //)
    //}
//}

impl Tilix {
    fn install(context: &Context) -> Result<()> {
        let Context {ref prompt, is_dry_run} = context;

        let dconf_path = DOTFILES_TILIX_DCONF_PATH.try_into_path()?;
        prompt.proc_with(
            &format!("load tilix dconf file from {}", dconf_path.to_string_lossy()),
            || {
                if !is_dry_run {
                    let file = OpenOptions::new()
                        .read(true)
                        .open(&dconf_path)?;

                    Command::new("dconf")
                        .arg("load")
                        .arg("/com/gexperts/Tilix/")
                        .stdin(file)
                        .spawn()?;
                }
                Ok(())
            }
        )
    }
}


impl Tilix {
    fn save(context: &Context) -> Result<()> {
        let Context {ref prompt, is_dry_run} = context;
        let dconf_path = DOTFILES_TILIX_DCONF_PATH.try_into_path()?;

        prompt.proc_with(
            &format!("dump tilix dconf file into {}", dconf_path.to_string_lossy()),
            || {
                if !is_dry_run {
                    let file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(&dconf_path)?;

                    Command::new("dconf")
                        .arg("dump")
                        .arg("/com/gexperts/Tilix/")
                        .stdout(file)
                        .spawn()?;
                }
                Ok(())
            }
        )
            
    }
}

impl Runner for Tilix {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("tilix")
            .about("Setting configuration files of tmux")
            .subcommand(
                SubCommand::with_name("install")
                    .about("load tilix configuration files")
            )
            .subcommand(
                SubCommand::with_name("save")
                    .about("dump and save tilix configuration files")
            )
    }

    fn run(argm: &ArgMatches, context: &Context) -> Result<()> {

        match argm.subcommand_name() {
            Some(name) => match name {
                "install" => {
                    Self::install(context)?;
                }
                "save" => {
                    Self::save(context)?;
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
