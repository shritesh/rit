use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { path: Option<PathBuf> },
}

fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();

    match args.command {
        Commands::Init { path } => {
            let root_path = match path {
                Some(p) => Ok(p),
                None => std::env::current_dir(),
            };

            let git_path = root_path?.join(".git").canonicalize()?;

            for dir in ["objects", "refs"] {
                std::fs::create_dir_all(git_path.join(dir))?;
            }

            println!("Initialized empty Rit repository in {}", git_path.display());
        }
    }

    Ok(())
}
