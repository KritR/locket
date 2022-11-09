use clap::Clap;
use std::path::PathBuf;
mod commands;

#[derive(Clap)]
#[clap(
    version = "1.0",
    about = "The simple secret manager for github/gitlab/and bitbucket",
    author = "Krithik R. <krdevmail@gmail.com>"
)]

enum Locket {
    #[clap(about = "Initializes locket secret management in this repo")]
    Init {
        #[clap(short, long, parse(from_os_str))]
        dir: Option<PathBuf>,
        #[clap(short, long)]
        remote: Option<String>,
    },
    #[clap(about = "Checks the status of the locked files in the repo")]
    Status,
    #[clap(about = "Adds a file to be managed by locket")]
    Add {
        #[clap(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    #[clap(about = "Removes a file to be managed by locket")]
    Rm {
        #[clap(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    #[clap(about = "Locks/encrypts the specified files in the repo")]
    Lock {
        #[clap(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    #[clap(about = "Unlock/decryptes the specified files in the repo")]
    Unlock {
        #[clap(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
}

fn main() {
    let opts: Locket = Locket::parse();

    let res: Result<(), String> = match opts {
        Locket::Init { dir, remote } => commands::init(dir, remote),
        Locket::Status => commands::status(),
        Locket::Add { files } => commands::add(&files),
        Locket::Rm { files } => commands::rm(&files),
        Locket::Lock { files } => commands::lock(&files),
        Locket::Unlock { files } => commands::unlock(&files),
    };

    if res.is_err() {
        println!("An error occurred");
        println!("{}", res.err().unwrap());
    }
}
