use std::path::{Path, PathBuf};
use std::fs::{self, DirBuilder};
use std::os;
use slog_scope;

mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
            VarError(::std::env::VarError);
        }

        errors {
            EnvVarNotSet(var: String) {
                description("env var not set")
                display("env var {} is not set", var)
            }
        }
    }
}

pub use self::errors::*;

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

pub fn get_home_dir() -> Result<PathBuf> {
    use std::env::{self, VarError};

    env::var("HOME")
        .map(PathBuf::from)
        .map_err(|e| {
            match e {
                VarError::NotPresent =>
                    Error::from_kind(ErrorKind::EnvVarNotSet("HOME".to_owned())),
                other => other.into()
            }
        })
}

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let logger = slog_scope::logger();
    info!(logger,
          "Making symlink {} -> {}",
          dst.as_ref().to_string_lossy(),
          src.as_ref().to_string_lossy());

    os::unix::fs::symlink(&src, &dst).map_err(
        |e| Error::with_chain(e, format!("Symlink {} -> {} failed",
                                         dst.as_ref().to_string_lossy(),
                                         src.as_ref().to_string_lossy()))
    )
}
