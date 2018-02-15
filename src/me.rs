use std::{
    env,
    fs,
    os,
    io,
    path::Path,
};
use clap::{ArgMatches, SubCommand, App};

use Runner;
use ops;
use ops::path::ensure_parent_dir;
use slog::Logger;

use git2::{
    self,
    Repository,
    ADD_DEFAULT,
    ADD_CHECK_PATHSPEC,
};

use common::*;

pub struct Me {}

lazy_static! {
    static ref INSTALL_DIR: HomePath = HomePath(".config/dotfiles".into());
}

impl Me {
    fn commit() -> Result<()> {
        let repo = Repository::open(INSTALL_DIR.try_into_path()?)?;
        let mut index = repo.index()?;

        index.add_all(
            &["*"],
            ADD_DEFAULT | ADD_CHECK_PATHSPEC,
            None,
        )?;

        let tree_oid = index.write_tree()?;
        let tree = repo.find_tree(tree_oid).unwrap();

        let parent = repo.find_commit(repo.refname_to_id("hao123")?)?;

        let signature = repo.signature()?;

        let mut commit_message = String::new();
        print!("Type commit message here: ");
        io::stdin().read_line(&mut commit_message)?;

        repo.commit(
            Some("hao123"),
            &signature,
            &signature, 
            &commit_message,
            &tree,
            &[&parent],
        )?;

        



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

    fn upgrade() -> Result<()> {
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
                SubCommand::with_name("upgrade")
                    .about("pull and upgrade"),
            )
            .subcommand(
                SubCommand::with_name("commit")
                    .about("save changes and commit"),
            )
    }

    fn run(argm: &ArgMatches, logger: Logger) -> Result<()> {
        match argm.subcommand_name().expect("No subcommand found") {
            "upgrade" => {
                Self::upgrade()?;
            },
            "install" => {
                Self::commit()?;
            },
            _ => unreachable!(),
        };
        Ok(())
    }
}
