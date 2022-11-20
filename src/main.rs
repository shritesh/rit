mod author;
mod blob;
mod commit;
mod database;
mod entry;
mod object;
mod tree;
mod workspace;

use chrono::Local;
use clap::{Parser, Subcommand};
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{stdin, Read, Write},
    path::PathBuf,
};

use author::Author;
use blob::Blob;
use commit::Commit;
use database::Database;
use entry::Entry;
use object::{OId, Object};
use tree::Tree;
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

fn main() -> Result<(), Box<dyn Error>> {
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

            let entries = workspace
                .list_files()?
                .into_iter()
                .map(|path| {
                    let data = workspace.read_file(&path)?;
                    let blob = Blob::new(data);

                    let blob_oid = database.store(blob)?;
                    Ok(Entry::new(path, blob_oid))
                })
                .collect::<Result<Vec<Entry>, std::io::Error>>()?;

            let tree = Tree::new(entries);
            let tree_oid = database.store(tree)?;

            let name = std::env::var("GIT_AUTHOR_NAME")?;
            let email = std::env::var("GIT_AUTHOR_EMAIL")?;
            let author = Author::new(name, email, Local::now());

            let mut message = String::new();
            stdin().read_to_string(&mut message)?;

            let commit = Commit::new(tree_oid, author, &message);
            let commit_oid = database.store(commit)?;

            let mut file = File::create(git_path.join("HEAD"))?;
            file.write_all(commit_oid.as_hexstr().as_bytes())?;

            println!(
                "[(root-commit)] {} {}",
                commit_oid.as_hexstr(),
                message.lines().next().unwrap_or_default()
            )
        }
    }

    Ok(())
}
