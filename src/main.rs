use std::{env, error::Error};

use git2::{Branch, BranchType, Repository};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Default)]
struct BranchInfo {
    name: String,
    local: bool,
    current: bool,
}

impl BranchInfo {
    fn from(branch: (Branch, BranchType)) -> Option<Self> {
        if let Ok(Some(name)) = branch.0.name() {
            return Some(Self {
                name: name.to_string(),
                local: branch.1 == BranchType::Local,
                current: branch.0.is_head(),
            });
        }

        None
    }
}

fn get_local_branches(repo: &Repository) -> Result<Vec<BranchInfo>> {
    let branches = repo
        .branches(Some(BranchType::Local))?
        .filter_map(std::result::Result::ok)
        .map(BranchInfo::from)
        .flatten()
        .collect::<Vec<BranchInfo>>();

    Ok(branches)
}

fn switch_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    // repo.set_head(&("refs/heads/".to_owned() + branch_name));
    todo!("'git switch' try repo set_head")
}

fn main() -> Result<()> {
    let mut args = env::args();
    let repo_path = args.nth(1).unwrap();
    let repo = Repository::open(&repo_path)?;
    let branches = get_local_branches(&repo)?;
    let current_branch = branches.iter().find(|b| b.current);
    println!("local branches: {:?}", branches);
    println!("current branch: {:?}", current_branch);
    Ok(())
}
