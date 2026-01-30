use clap::{Parser, Subcommand};

mod commands;
mod config;

#[derive(Parser)]
#[command(name = "mdlibs")]
#[command(
    author,
    version,
    about = "A markdown library and document management CLI tool"
)]
#[command(
    long_about = "mdlibs is a command-line tool for managing collections of markdown documents.\nIt provides functionality to initialize libraries, list documents, update metadata, and search through your markdown files."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new markdown library
    Init {
        /// Path where to initialize the library (defaults to current directory)
        #[arg(default_value = ".")]
        path: String,
    },
    /// List all markdown documents in the library
    List {
        /// Filter documents by title or path content
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Update metadata or content of a markdown document
    Update {
        /// Document path or name (with or without .md extension)
        document: String,
        /// New title for the document
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Search through markdown documents
    Search {
        /// Search query (case-insensitive)
        query: String,
        /// Search only in document titles
        #[arg(short, long)]
        title_only: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Init { path } => commands::init::run(path),
        Commands::List { filter } => commands::list::run(filter.as_deref()),
        Commands::Update { document, title } => commands::update::run(document, title.as_deref()),
        Commands::Search { query, title_only } => commands::search::run(query, *title_only),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
