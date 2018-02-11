use std::path::{PathBuf, Path};
use common::*;

pub fn get_home_dir() -> Result<PathBuf> {
    use std::env::{self, VarError};

    Ok(env::var("HOME")
        .map(PathBuf::from)
        .context("Env var $HOME not set")?)
        //.map_err(|e| -> Error { match e {
                //VarError::NotPresent => "env var $HOME not set".into(),
                //other => other.into(),
            //}
        //})
}

pub struct HomePath(pub PathBuf);

impl HomePath {
    pub fn try_into_path(&self) -> Result<PathBuf> {
        let home_dir = get_home_dir()?;
        Ok(home_dir.join(&self.0))
    }

    pub fn join<T: AsRef<Path>>(&self, path: T) -> HomePath {
        HomePath(self.0.join(path))
    }
}
