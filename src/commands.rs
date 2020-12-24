use dialoguer::{theme::ColorfulTheme, Select};
use git2::Repository;
use std::env;
use std::path::PathBuf;

pub fn init(dir: Option<PathBuf>, remote: Option<String>) -> Result<(), String> {
    let repo_dir = dir
        .ok_or(0)
        .or(env::current_dir())
        .or_else(|_| Err("Failed to read the current directory."))?;

    let repo = Repository::discover(repo_dir).or_else(|_| {
        Err("Failed to find git repository. Ensure that the directory is a git repo.")
    })?;

    println!("Repo path is: {}", repo.path().parent().unwrap().display());

    let selected_remote = remote
        .ok_or("No remote provided")
        .or_else(|_e| prompt_repo_remote(&repo))?;

    let repo_remote = repo
        .find_remote(&selected_remote)
        .or(Err("Failed to find selected remote"))?;

    println!("Selected repo remote is {}", repo_remote.name().unwrap());

    println!("Init Repo");
    Ok(())
}

fn prompt_repo_remote(repo: &Repository) -> Result<String, String> {
    let remotes = repo
        .remotes()
        .or(Err("Repository doesn't contain remotes".to_string()))?;

    if remotes.len() < 2 {
        return remotes
            .get(0)
            .ok_or("Repository has no valid remotes".to_string())
            .and_then(|a| Ok(a.to_string()));
    }

    let options: Vec<String> = remotes
        .iter()
        .filter_map(|x| x)
        .map(|p| p.to_string())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Optionally pick your flavor")
        .default(0)
        .items(&options[..])
        .interact_opt()
        .unwrap()
        .ok_or("No remote selection provided".to_string())?;

    return options
        .get(selection)
        .ok_or("Bad Selection".to_string())
        .and_then(|i| Ok(i.to_string()));
}

pub fn status() {
    println!("Repo Status")
}

pub fn add(files: &Vec<PathBuf>) {
    println!("Adding to Repo");
}

pub fn rm(files: &Vec<PathBuf>) {
    println!("Removing from Repo");
}

pub fn lock(files: &Vec<PathBuf>) {
    println!("Locking the Repo");
}

pub fn unlock(files: &Vec<PathBuf>) {
    println!("Unlocking the Repo");
}
