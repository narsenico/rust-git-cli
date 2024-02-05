use std::error::Error;

use git2::{Branch, BranchType, Repository};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Default)]
pub struct BranchInfo {
    pub name: String,
    pub local: bool,
    pub current: bool,
}

fn parse_branch(branch: (Branch, BranchType)) -> Option<BranchInfo> {
    if let Ok(Some(name)) = branch.0.name() {
        return Some(BranchInfo {
            name: name.to_string(),
            local: branch.1 == BranchType::Local,
            current: branch.0.is_head(),
        });
    }

    None
}

fn get_branches(repo: &Repository, branch_type: Option<BranchType>) -> Result<Vec<BranchInfo>> {
    let branches = repo
        .branches(branch_type)?
        .filter_map(std::result::Result::ok)
        .map(parse_branch)
        .flatten()
        .collect::<Vec<BranchInfo>>();

    Ok(branches)
}

pub fn get_local_branches(repo: &Repository) -> Result<Vec<BranchInfo>> {
    get_branches(repo, Some(BranchType::Local))
}
