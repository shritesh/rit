use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { path: Option<PathBuf> },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init { path } => {
            let git_path = path
                .unwrap_or_default()
                .join(".git")
                .canonicalize()
                .unwrap();

            for dir in ["objects", "refs"] {
                std::fs::create_dir_all(git_path.join(dir)).unwrap();
            }

            println!("Initialized empty Rit repository in {}", git_path.display());
        }
    }
}
