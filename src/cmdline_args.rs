use super::Result;
use anyhow::anyhow;
use clap::Parser;
use std::path::PathBuf;

#[rustfmt::skip]
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short = 'g', long, help = "git or https url for repository", value_name = "URL")]
    pub git_url: String,
    #[arg(short = 'd', long, help = "destination path where to clone the repository", value_name = "PATH")]
    pub destination: PathBuf,
}

impl Args {
    pub fn init() -> Self {
        let mut args = Args::parse();
        if let Ok(repo_name) = get_repo_name(&args.git_url) {
            if !args.destination.ends_with(repo_name) {
                args.destination.push(repo_name);
            }
        }
        args
    }
}

fn get_repo_name(git_url: &str) -> Result<&str> {
    let (_, name) = git_url
        .rsplit_once('/')
        .ok_or_else(|| anyhow!("Cannot get repository name from the url"))?;
    Ok(name.trim_end_matches(".git"))
}
