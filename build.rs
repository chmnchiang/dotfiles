use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Cargo didn't set $OUT_DIR"));

    let home_dir = PathBuf::from(env::var("HOME").expect("Env var $HOME is not set"));

    let target_dir = home_dir.join(".config/dotfiles2");

    let parent_dir = target_dir.parent().unwrap();
    fs::DirBuilder::new().recursive(true).create(parent_dir).unwrap();
    
    fs::copy(&out_dir, &target_dir);
}
