use std::{
    io::{
        self,
        Write,
    },
    process::Command,
};
use clap::{ArgMatches, SubCommand, App};

use git2::{
    Repository,
    IndexAddOption,
};

use crate::common::*;

pub struct Me {}

lazy_static! {
    pub static ref INSTALL_DIR: HomePath = HomePath(".config/dotfiles".into());
}

impl Me {
    fn commit(Context {ref prompt, is_dry_run}: &Context) -> Result<()> {
        let path = INSTALL_DIR.try_into_path()?;

        let repo = Repository::open(&path)?;
        let mut index = repo.index()?;

        prompt.info("stage all changes and commit, (a.k.a `git add -A; git commit`)");

        if !is_dry_run {
            index.add_all(
                &["*"],
                IndexAddOption::DEFAULT | IndexAddOption::CHECK_PATHSPEC,
                None,
            )?;
            index.write()?;

            let tree_oid = index.write_tree()?;
            let tree = repo.find_tree(tree_oid).unwrap();
            let parent = repo.find_commit(repo.refname_to_id("HEAD")?)?;
            let signature = repo.signature()?;

            let mut commit_message = String::new();
            print!("Type commit message here: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut commit_message)?;

            repo.commit(
                Some("HEAD"),
                &signature,
                &signature, 
                &commit_message,
                &tree,
                &[&parent],
            )?;
        }
        
        prompt.info("push to remote");


        if !is_dry_run {
            let status = Command::new("git")
                .arg("-C")
                .arg(&path)
                .arg("push")
                .status()?;

            if !status.success() {
                bail!("push repo failed");
            }
        }

        Ok(())
    }

    fn upgrade(Context {ref prompt, is_dry_run}: &Context) -> Result<()> {
        let path = INSTALL_DIR.try_into_path()?;

        prompt.proc_with(
            &format!("pull remote into {}", path.to_string_lossy()),
            || {
                if !is_dry_run {
                    let status = Command::new("git")
                        .arg("-C")
                        .arg(&path)
                        .arg("pull")
                        .status()?;

                    if !status.success() {
                        bail!("pull repo failed");
                    }
                }
                Ok(())
            }
        )
    }

    fn dir(_: &Context) -> Result<()> {
        let path = INSTALL_DIR.try_into_path()?;
        println!("{}", path.to_string_lossy());
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
            .subcommand(
                SubCommand::with_name("dir")
                    .about("print the installation dir"),
            )
    }

    fn run(argm: &ArgMatches, context: &Context) -> Result<()> {
        match argm.subcommand_name() {
            Some(name) => generate_command!(name, context; upgrade, commit, dir),
            None => {
                Self::build_cli().print_help().unwrap();
                println!();
            }
        };
        Ok(())
    }
}
