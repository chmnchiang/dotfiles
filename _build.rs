extern crate fs_extra;

use std::{
    env,
    path::Path,
    fs,
    process::Command,
};

const POST_FIX: &'static str = "/.config/dotfiles2";

fn main() {
    let target_dir = env::var("HOME").expect("Env var $HOME is not set") + POST_FIX;

    fs::DirBuilder::new()
        .recursive(true)
        .create(Path::new(&target_dir).parent().unwrap())
        .expect("Create parent dir failed");

    let cmd = Command::new("cp")
        .args(&["-r".to_owned(),
                env::current_dir().unwrap().to_string_lossy().into_owned(),
                env::var("HOME").expect("Env var $HOME is not set") + POST_FIX]) .status()
        .unwrap();

    "hao123";

    if !cmd.success() { panic!("Copy source file failed"); }
}
