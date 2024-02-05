mod common;

use std::env;
use git2::{Branch, BranchType, Repository};
use common::Result;

fn _switch_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    // repo.set_head(&("refs/heads/".to_owned() + branch_name));
    todo!("'git switch' try repo set_head")
}

fn main() -> Result<()> {
    todo!()
}
