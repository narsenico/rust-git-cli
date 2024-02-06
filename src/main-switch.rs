mod common;

use std::process::ExitCode;

use common::{get_repo_path, Result};
use dialoguer::Select;
use git2::Repository;

use crate::common::{get_exe_name, get_local_branches};

fn switch_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    repo.set_head(&("refs/heads/".to_owned() + branch_name))?;
    Ok(())
}

fn main() -> Result<ExitCode> {
    let Some(repo_path) = get_repo_path() else {
        eprintln!("Usage: {exe} [--path <repo path>]", exe = get_exe_name()?);
        return Ok(ExitCode::FAILURE);
    };
    let repo = Repository::open(&repo_path)?;
    let branches = get_local_branches(&repo)?;
    let names = branches
        .iter()
        .filter(|b| !b.current)
        .map(|b| b.name.as_str())
        .collect::<Vec<&str>>();
    let names = names.as_slice();

    if names.len() == 0 {
        eprintln!("There is no branch to switch to");
        return Ok(ExitCode::FAILURE);
    }

    let choice = Select::new()
        .with_prompt("Select branch to switch into")
        .items(&names)
        .default(0)
        .interact_opt()?;

    let Some(choice) = choice else {
        println!("Switch cancelled");
        return Ok(ExitCode::SUCCESS);
    };

    if let Some(selected_branch) = branches.get(choice) {
        switch_branch(&repo, selected_branch.name.as_str())?;
    }

    Ok(ExitCode::SUCCESS)
}
