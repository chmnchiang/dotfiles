use crate::common::*;
use std::path::PathBuf;
use super::{
    Op,
};
use super::path::{
    ensure_not_exists,
    ensure_parent_dir,
};
use git2::Repository;

pub struct GitClone {
    pub url: String,
    pub dst: PathBuf,
}

impl Op for GitClone {
    fn commit(&self, context: &Context) -> Result<()> {
        ensure_parent_dir(context, &self.dst)?;
        ensure_not_exists(context, &self.dst)?;

        context.prompt.proc_with(
            &format!("cloning repo {} into {}", &self.url, self.dst.to_string_lossy()),
            || {
                if !context.is_dry_run {
                    Repository::clone(&self.url, &self.dst)?;
                }
                Ok(())
            }
        )
    }
}
