use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the monorepo root.
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Fix the issues automatically, if possible.
    #[arg(long)]
    pub fix: bool,

    /// Ignore the `multiple-dependency-versions` rule for the given dependency name and/or version.
    #[arg(long, short)]
    pub ignore_dependency: Vec<String>,

    /// Ignore rules for the given package name or path.
    #[arg(long, short = 'p')]
    pub ignore_package: Vec<String>,

    /// Ignore the given rule.
    #[arg(long, short = 'r')]
    pub ignore_rule: Vec<String>,
}
