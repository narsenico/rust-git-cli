mod common;

use std::process::ExitCode;

use common::{get_local_branches, get_repo_path, BranchInfo, Result};
use git2::Repository;

use crate::common::get_exe_name;

fn print_branch(branch: &BranchInfo) {
    println!(
        "{current} {name}",
        current = if branch.current { "*" } else { " " },
        name = branch.name
    );
}

fn main() -> Result<ExitCode> {
    let Some(repo_path) = get_repo_path() else {
        eprintln!("Usage: {exe} [--path <repo path>]", exe = get_exe_name()?);
        return Ok(ExitCode::FAILURE);
    };
    let repo = Repository::open(&repo_path)?;
    let branches = get_local_branches(&repo)?;
    for branch in branches {
        print_branch(&branch);
    }

    Ok(ExitCode::SUCCESS)
}
