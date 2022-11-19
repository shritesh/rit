mod blob;
mod database;
mod object;
mod workspace;

use clap::{Parser, Subcommand};
use std::{env, fs, io::Error, path::PathBuf};

use blob::Blob;
use database::Database;
use object::Object;
use workspace::Workspace;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { path: Option<PathBuf> },
    Commit,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();

    match args.command {
        Commands::Init { path } => {
            let root_path = match path {
                Some(p) => Ok(p),
                None => env::current_dir(),
            }?
            .canonicalize()?;

            let git_path = root_path.join(".git");

            for dir in ["objects", "refs"] {
                fs::create_dir_all(git_path.join(dir))?;
            }

            println!("Initialized empty Rit repository in {}", git_path.display());
        }

        Commands::Commit => {
            let root_path = env::current_dir()?;
            let git_path = root_path.join(".git");
            let db_path = git_path.join("objects");

            let workspace = Workspace::new(root_path);
            let database = Database::new(db_path);

            for path in workspace.list_files()? {
                let data = workspace.read_file(&path)?;
                let blob = Blob::new(data);

                database.store(blob)?;
            }
        }
    }

    Ok(())
}
