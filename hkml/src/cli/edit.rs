mod merge;
mod mirror;
mod rebase;

use clap::Subcommand;

use super::{InArgs, OutArgs, Run};
use crate::impl_run_inner;

use merge::*;
use mirror::*;
use rebase::*;

#[derive(Subcommand, Debug, Clone)]
pub enum Edit {
    /// Merge two modlinks
    Merge(Merge),
    /// Change mod download links to another base url
    Rebase(Rebase),
    /// Create a local mirror of modlinks
    Mirror(Mirror),
}

impl_run_inner! {
    Edit;
    Merge,
    Rebase,
    Mirror
}
