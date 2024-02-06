mod common;

use std::process::ExitCode;

use common::{get_local_branches, BranchInfo, Result};
use git2::Repository;

fn print_branch(branch: &BranchInfo) {
    println!(
        "{current} {name}",
        current = if branch.current { "*" } else { " " },
        name = branch.name
    );
}

fn main() -> Result<ExitCode> {
    let repo_path = ".";
    let repo = Repository::open(&repo_path)?;
    let branches = get_local_branches(&repo)?;
    for branch in branches {
        print_branch(&branch);
    }

    Ok(ExitCode::SUCCESS)
}
