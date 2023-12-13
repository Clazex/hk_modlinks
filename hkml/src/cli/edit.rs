mod merge;

use clap::Subcommand;

use super::{InArgs, OutArgs, Run};
use crate::impl_run_inner;

use merge::*;

#[derive(Subcommand, Debug, Clone)]
pub enum Edit {
    /// Merge two modlinks
    Merge(Merge),
}

impl_run_inner! {
    Edit;
    Merge
}
