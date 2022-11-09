use dialoguer::{theme::ColorfulTheme, Select};
use git2::{Remote, Repository};
use git_url_parse::GitUrl;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use webbrowser;

enum LocketGitService {
    Bitbucket,
    Github,
    Gitlab,
}

struct LocketRemote {
    service: LocketGitService,
    user: String,
    project: String,
}

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

    let locket_remote = parse_remote(&repo_remote)?;

    let user_token = validate_user(&locket_remote.service)?;

    //initialize_locket_repo(repo.path().parent().unwrap())?;

    println!("Selected repo remote is {}", repo_remote.name().unwrap());

    println!("Init Repo");
    Ok(())
}

fn parse_remote(repo_remote: &Remote) -> Result<LocketRemote, String> {
    let url = repo_remote.url().ok_or("Failed to read the remote url")?;

    let giturl = GitUrl::parse(&url).or(Err("Failed to parse URL"))?;

    let service = match giturl.host.ok_or("No service matched")?.as_str() {
        "github.com" => LocketGitService::Github,
        "gitlab.com" => LocketGitService::Gitlab,
        "bitbucket.org" => LocketGitService::Bitbucket,
        _ => {
            return Err(String::from("Invalid service matched"));
        }
    };

    let remote: LocketRemote = LocketRemote {
        service: service,
        user: giturl.owner.ok_or("No owner identified")?,
        project: giturl.name,
    };

    Ok(remote)
}

fn validate_user(service: &LocketGitService) -> Result<String, String> {
    let conf = read_locket_config()?;
    match service {
        LocketGitService::Github => Ok(String::from("Token")),
        _ => Ok(String::from("Token")),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct LocketConfig {
    githubToken: Option<String>,
    gitlabToken: Option<String>,
    bitbucketToken: Option<String>,
}

fn read_locket_config() -> Result<LocketConfig, String> {
    let conf = LocketConfig {
        githubToken: None,
        gitlabToken: None,
        bitbucketToken: None,
    };
    return Ok(conf);
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

pub fn status() -> Result<(), String> {
    println!("Repo Status");
    Ok(())
}

pub fn add(files: &Vec<PathBuf>) -> Result<(), String> {
    println!("Adding to Repo");
    Ok(())
}

pub fn rm(files: &Vec<PathBuf>) -> Result<(), String> {
    println!("Removing from Repo");
    Ok(())
}

pub fn lock(files: &Vec<PathBuf>) -> Result<(), String> {
    println!("Locking the Repo");
    Ok(())
}

pub fn unlock(files: &Vec<PathBuf>) -> Result<(), String> {
    println!("Unlocking the Repo");
    Ok(())
}
