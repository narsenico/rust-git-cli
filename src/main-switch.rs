mod common;

use std::process::ExitCode;

use common::Result;
use dialoguer::Select;
use git2::Repository;

use crate::common::get_local_branches;

fn switch_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    repo.set_head(&("refs/heads/".to_owned() + branch_name))?;
    Ok(())
}

fn main() -> Result<ExitCode> {
    let repo_path = ".";
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
