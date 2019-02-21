use std::{
    path::{Path, PathBuf},
    fs::{self, DirBuilder},
    os,
};

use crate::common::*;
use super::{
    Op,
};

pub struct EnsureParentDir(pub PathBuf);

impl Op for EnsureParentDir {
    fn commit(&self, context: &Context) -> Result<()> {
        ensure_parent_dir(context, &self.0)
    }
}

pub fn ensure_dir<T: AsRef<Path>>(&Context {ref prompt, is_dry_run}: &Context, path: T) -> Result<()> {
    prompt.proc_with(
        &format!("ensuring directory {} exists", path.as_ref().to_string_lossy()),
        || {
            if !is_dry_run {
                DirBuilder::new().recursive(true).create(&path)?;
            }
            Ok(())
        }
    )
}

pub fn ensure_parent_dir<T: AsRef<Path>>(context: &Context, path: T) -> Result<()> {
    match path.as_ref().parent() {
        Some(parent_path) => ensure_dir(context, parent_path),
        None => Ok(()),
    }
}

pub fn ensure_not_exists<T: AsRef<Path>>(context: &Context, path: T) -> Result<()> {
    let &Context {ref prompt, is_dry_run} = context;
    let path = path.as_ref();

    if !path.exists() {
        return Ok(());
    }

    let path_string = path.to_string_lossy();

    let answer = prompt.ask_yesno(&format!("path {} exists, remove it?", path_string));

    if !answer {
        bail!("can't proceed because {} exists", path_string);
    }

    prompt.proc_with(
        &format!("removing {}", path_string),
        || {
            if !is_dry_run {
                if path.is_dir() {
                    fs::remove_dir_all(path)?;
                } else {
                    fs::remove_file(path)?;
                }
            }
            Ok(())
        }
    )
}

pub struct SymLink {
    pub src: PathBuf,
    pub dst: PathBuf,
}

impl Op for SymLink {
    fn commit(&self, context: &Context) -> Result<()> {
        symlink(context, &self.src, &self.dst)
    }
}

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(context: &Context, src: P, dst: Q) -> Result<()> {
    ensure_parent_dir(context, dst.as_ref())?;
    ensure_not_exists(context, dst.as_ref())?;

    let prompt = &context.prompt;
    prompt.proc_with(
        &format!(
            "Making symlink {} -> {}",
            dst.as_ref().to_string_lossy(),
            src.as_ref().to_string_lossy()
        ),
        || {
            if !context.is_dry_run {
                os::unix::fs::symlink(&src, &dst).map_err(
                    |_| format_err!("Symlink {} -> {} failed", dst.as_ref().to_string_lossy(), src.as_ref().to_string_lossy())
                )?;
            }
            Ok(())
        }
    )
}
