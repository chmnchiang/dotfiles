use std::path::{PathBuf, Path};
use crate::common::*;
use crate::me::INSTALL_DIR;

lazy_static! {
    static ref HOME_PATH: PathBuf = {
        use std::env;
        env::var("HOME")
            .map(PathBuf::from)
            .context("Env var $HOME not set")
            .unwrap()
    };
}

#[derive(Clone)]
pub struct HomePath(pub PathBuf);

impl HomePath {
    //pub fn into_path(&self) -> PathBuf {
        //HOME_PATH.join(&self.0)
    //}

    pub fn try_into_path(&self) -> Result<PathBuf> {
        Ok(HOME_PATH.join(&self.0))
    }

    pub fn join<T: AsRef<Path>>(&self, path: T) -> HomePath {
        HomePath(self.0.join(path))
    }
}

impl From<HomePath> for PathBuf {
    fn from(hp: HomePath) -> PathBuf {
        HOME_PATH.join(&hp.0)
    }
}

impl From<&HomePath> for PathBuf {
    fn from(hp: &HomePath) -> PathBuf {
        HOME_PATH.join(&hp.0)
    }
}

#[derive(Clone)]
pub struct DotfilesPath(pub PathBuf);

impl From<DotfilesPath> for PathBuf {
    fn from(dp: DotfilesPath) -> PathBuf {
        INSTALL_DIR.join("dotfiles").join(&dp.0).into()
    }
}

impl From<&DotfilesPath> for PathBuf {
    fn from(dp: &DotfilesPath) -> PathBuf {
        INSTALL_DIR.join("dotfiles").join(&dp.0).into()
    }
}
