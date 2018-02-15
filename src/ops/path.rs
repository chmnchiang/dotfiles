use std::{
    path::Path,
    fs::{self, DirBuilder},
    os,
};

use common::*;
use super::context::Context;

pub fn ensure_dir<T: AsRef<Path>>(&Context {ref logger, is_dry_run}: &Context, path: T) -> Result<()> {
    info!(logger, "create directory {} if not exists", path.as_ref().to_string_lossy());

    if !is_dry_run {
        DirBuilder::new().recursive(true).create(&path)?;
    }
    Ok(())
}

pub fn ensure_parent_dir<T: AsRef<Path>>(context: &Context, path: T) -> Result<()> {
    match path.as_ref().parent() {
        Some(parent_path) => ensure_dir(context, parent_path),
        None => Ok(()),
    }
}

pub fn ensure_clean<T: AsRef<Path>>(context: &Context, path: T) -> Result<()> {
    let &Context {ref logger, is_dry_run} = context;

    info!(logger,
          "Cleaning {}",
          path.as_ref().to_string_lossy());

    if !is_dry_run {
        if !path.as_ref().exists() {
            return Ok(());
        }

        if path.as_ref().is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(context: &Context, src: P, dst: Q) -> Result<()> {
    let logger = &context.logger;
    info!(logger,
          "Making symlink {} -> {}",
          dst.as_ref().to_string_lossy(),
          src.as_ref().to_string_lossy());

    ensure_parent_dir(context, dst.as_ref())?;

    if !context.is_dry_run {
        os::unix::fs::symlink(&src, &dst).map_err(
            |_| format_err!("Symlink {} -> {} failed", dst.as_ref().to_string_lossy(), src.as_ref().to_string_lossy())
        )
    } else {
        Ok(())
    }
}
