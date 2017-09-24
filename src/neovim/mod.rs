use std::env;
use std::path::{Path, PathBuf};
use std::fs::DirBuilder;
use std::os;
use clap::{Arg, SubCommand, App};

use git2::Repository;

const DEIN_URL: &'static str = "https://github.com/Shougo/dein.vim";
pub struct Neovim { }

impl ::Installer for Neovim {


    fn build_cli() -> App<'static, 'static> {
        SubCommand::with_name("neovim")
            .about("Setting config of neovim")
    }

    #[cfg(target_os = "linux")]
    fn run() {
        let home_dir = env::var("HOME").expect("env var $HOME not set.");
        let home_dir = Path::new(&home_dir).to_owned();
        
        let mut vim_dir = home_dir.clone();
        vim_dir.push(".vim");

        // Make .vim/ and clone dein.vim
        DirBuilder::new()
            .recursive(true)
            .create(&vim_dir).expect(
                &format!("create dir {} failed", vim_dir.to_string_lossy())
            );

        // Clone dein.vim
        let mut dein_dir = vim_dir;
        dein_dir.push("dein.vim");
        Repository::clone(DEIN_URL, &dein_dir).expect(
            &format!("Cloning dein.vim to {} failed", dein_dir.to_string_lossy()));

        // Make nvim config dir
        let mut config_dir = home_dir.clone();
        config_dir.push(".config");

        DirBuilder::new()
            .recursive(true)
            .create(&config_dir)
            .expect(&format!("create dir {} failed", config_dir.to_string_lossy()));

        let mut nvim_dir = config_dir;
        nvim_dir.push("nvim");

        let mut dotfiles_nvim_dir = env::current_dir().unwrap();
        dotfiles_nvim_dir.push("dotfiles");
        dotfiles_nvim_dir.push("nvim");

        os::unix::fs::symlink(&dotfiles_nvim_dir, &nvim_dir)
            .expect(&format!("Create symlink {} -> {} failed",
                            dotfiles_nvim_dir.to_string_lossy(),
                            nvim_dir.to_string_lossy()));
    }

    #[cfg(not(target_os = "linux"))]
    fn run() {
        unimplemented!()
    }
}
