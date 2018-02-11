use std::{
    self,
    path::{Path, PathBuf},
    fs::{self, DirBuilder},
    os,
};

use slog_scope;
use common::*;

pub fn ensure_dir<T: AsRef<Path>>(path: T) -> Result<()> {
    let logger = slog_scope::logger();
    info!(logger, "ensuring directory {} exists", path.as_ref().to_string_lossy());

    Ok(DirBuilder::new().recursive(true).create(&path)?)
}

pub fn ensure_parent_dir<T: AsRef<Path>>(path: T) -> Result<()> {
    match path.as_ref().parent() {
        Some(parent_path) => ensure_dir(parent_path),
        None => Ok(()),
    }
}

pub fn ensure_clean<T: AsRef<Path>>(path: T) -> Result<()> {
    let logger = slog_scope::logger();
    info!(logger,
          "Cleaning {}",
          path.as_ref().to_string_lossy());
    
    if !path.as_ref().exists() {
        return Ok(());
    }

    if path.as_ref().is_dir() {
        Ok(fs::remove_dir_all(path)?)
    } else {
        Ok(fs::remove_file(path)?)
    }
}

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let logger = slog_scope::logger();
    info!(logger,
          "Making symlink {} -> {}",
          dst.as_ref().to_string_lossy(),
          src.as_ref().to_string_lossy());

    os::unix::fs::symlink(&src, &dst).map_err(
        |_| format_err!("Symlink {} -> {} failed", dst.as_ref().to_string_lossy(), src.as_ref().to_string_lossy())
    )
}
