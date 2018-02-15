use std::{
    env,
    fs,
    os,
    io::{
        self,
        Write,
    },
    path::Path,
    process::Command,
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
        let path = INSTALL_DIR.try_into_path()?;
        let repo = Repository::open(&path)?;
        let mut index = repo.index()?;

        index.add_all(
            &["*"],
            ADD_DEFAULT | ADD_CHECK_PATHSPEC,
            None,
        )?;

        index.write()?;
        let tree_oid = index.write_tree()?;

        let tree = repo.find_tree(tree_oid).unwrap();

        let parent = repo.find_commit(repo.refname_to_id("HEAD")?)?;

        let signature = repo.signature()?;

        let mut commit_message = String::new();
        print!("Type commit message here: ");
        io::stdout().flush();
        io::stdin().read_line(&mut commit_message)?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature, 
            &commit_message,
            &tree,
            &[&parent],
        )?;
        

        let status = Command::new("git")
            .arg("-C")
            .arg(&path)
            .arg("push")
            .status()?;

        if !status.success() {
            bail!("push repo failed");
        }

        Ok(())
    }

    fn upgrade() -> Result<()> {
        let path = INSTALL_DIR.try_into_path()?;

        let status = Command::new("git")
            .arg("-C")
            .arg(&path)
            .arg("pull")
            .status()?;

        if !status.success() {
            bail!("pull repo failed");
        }

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
            "commit" => {
                Self::commit()?;
            },
            _ => unreachable!(),
        };
        Ok(())
    }
}
