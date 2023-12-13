mod apply;
mod generate;

use clap::Subcommand;

use serde::{Deserialize, Serialize};

use super::{InArgs, OutArgs, Run};
use crate::impl_run_inner;

use apply::*;
use generate::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModTranslation {
    pub name: String,
    pub desc: String,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Translate {
    Apply(Apply),
    Generate(Generate),
}

impl_run_inner! {
    Translate;
    Apply,
    Generate
}
