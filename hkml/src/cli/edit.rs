mod merge;
mod mirror;
mod rebase;
mod translate;

use clap::Subcommand;

use super::{InArgs, OutArgs, Run};
use crate::impl_run_inner;

use merge::*;
use mirror::*;
use rebase::*;
use translate::*;

#[derive(Subcommand, Debug, Clone)]
pub enum Edit {
    /// Merge two modlinks
    Merge(Merge),
    /// Change mod download links to another base url
    Rebase(Rebase),
    /// Create a local mirror of modlinks
    Mirror(Mirror),
    /// Modlink translation
    #[command(subcommand)]
    Translate(Translate),
}

impl_run_inner! {
    Edit;
    Merge,
    Rebase,
    Mirror,
    Translate
}
