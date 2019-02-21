use std;
pub use failure::{
    Error,
    ResultExt,
    bail,
    format_err,
};

pub type Result<T> = std::result::Result<T, Error>;

pub use lazy_static::lazy_static;

pub use crate::common_path::*;

pub use slog::{Logger, error, info};

pub use crate::runner::{Context, Runner, Installable};
