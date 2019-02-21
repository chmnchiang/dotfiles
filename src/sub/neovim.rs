use clap::{ArgMatches, SubCommand, App};

use crate::ops::Op;
use crate::ops::Seq;
use crate::ops::path::*;
use crate::ops::git::*;

use crate::common::*;

const DEIN_URL: &str = "https://github.com/Shougo/dein.vim";


pub struct Neovim {}

impl Installable for Neovim {
    #[allow(non_snake_case)]
    fn generate_op() -> Box<dyn Op> {
        let DEIN_PATH = &HomePath(".vim/dein.vim".into());
        let NVIM_INSTALL_DIR = &HomePath(".config/nvim".into());
        let DOTFILES_NVIM_DIR = &DotfilesPath("nvim".into());

        Box::new(Seq::from((
            GitClone {
                url: DEIN_URL.into(),
                dst: DEIN_PATH.into(),
            },
            SymLink {
                src: DOTFILES_NVIM_DIR.into(),
                dst: NVIM_INSTALL_DIR.into(),
            }
        )))
    }
}

//impl Neovim {
    //fn install(context: Context) -> Result<()> {
        //NEOVIM_OP.commit(&context)
    //}
//}


//impl Neovim {
    //fn clean(context: Context) -> Result<()> {
        //NEOVIM_OP.revert(&context)
    //}
//}

impl Runner for Neovim {
    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("neovim")
            .about("Setting configuration files of neovim")
            .subcommand(
                SubCommand::with_name("install")
                    .about("install neovim configuration")
            )
            //.subcommand(
                //SubCommand::with_name("uninstall")
                    //.about("uninstall neovim configuration files")
            //)
    }

    fn run(argm: &ArgMatches, context: &Context) -> Result<()> {

        match argm.subcommand_name() {
            Some(name) => generate_command!(name, context; install),
            None => {
                Self::build_cli().print_help().unwrap();
                println!();
            }
        };
        Ok(())
    }
}
