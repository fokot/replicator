use std::env;
use std::path::Path;
use git2::Repository;

fn main() {
    let args: Vec<String> = env::args().collect();
    // last arg is the binary..
    if args.len() != 2 {
        println!("Usage: cargo run <git-url>");
        return;
    }

    let git_url = &args[1];
    let destination = get_destination(git_url);
    println!("Destination {}", destination);

    Repository::clone(git_url, Path::new(destination.as_str())).expect("Error cloning repo");

    println!("Repository cloned successfully to {}!", destination);
}

fn get_destination(git_url: &str) -> String {
    let mut destination = "git-repos/".to_owned();
    destination.push_str(get_repo_name(git_url));
    destination
}

fn get_repo_name(git_url: &str) -> &str {
    let split_index = git_url.rfind('/').unwrap_or(0) + 1;
    let (_, name) = git_url.split_at(split_index);
    if name.ends_with(".git") {
        name.trim_end_matches(".git")
    } else {
        name
    }
}
