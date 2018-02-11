use std;
pub use failure::{
    Error,
    ResultExt,
};

pub type Result<T> = std::result::Result<T, Error>;

pub use home_path::HomePath;
