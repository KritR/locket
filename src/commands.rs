use std::path::PathBuf;

pub fn init(dir: Option<PathBuf>) {
    println!("Init Repo");
}

pub fn status() {
    println!("Repo Status")
}

pub fn add(files: Vec<PathBuf>) {
    println!("Adding to Repo");
}

pub fn rm(files: Vec<PathBuf>) {
    println!("Removing from Repo");
}

pub fn lock(files: Vec<PathBuf>) {
    println!("Locking the Repo");
}

pub fn unlock(files: Vec<PathBuf>) {
    println!("Unlocking the Repo");
}
