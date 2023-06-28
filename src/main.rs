mod cmdline_args;

use anyhow::Result;
use git2::Repository;

fn main() -> Result<()> {
    let args = cmdline_args::Args::init();

    println!("Destination {}", args.destination.display());

    Repository::clone(&args.git_url, &args.destination)?;

    println!(
        "Repository cloned successfully to {}!",
        args.destination.display()
    );
    Ok(())
}
