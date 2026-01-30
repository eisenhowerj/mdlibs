use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mdlibs")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new markdown library
    Init {
        /// Path where to initialize the library
        #[arg(default_value = ".")]
        path: String,
    },
    /// List all markdown documents in the library
    List {
        /// Filter by tag or category
        #[arg(short, long)]
        filter: Option<String>,
    },
    /// Update metadata or content of markdown documents
    Update {
        /// Document identifier
        document: String,
        /// New title for the document
        #[arg(short, long)]
        title: Option<String>,
    },
    /// Search through markdown documents
    Search {
        /// Search query
        query: String,
        /// Search only in titles
        #[arg(short, long)]
        title_only: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { path } => {
            println!("Initializing markdown library at: {}", path);
            // TODO: Implement initialization logic
        }
        Commands::List { filter } => {
            if let Some(f) = filter {
                println!("Listing documents with filter: {}", f);
            } else {
                println!("Listing all documents");
            }
            // TODO: Implement list logic
        }
        Commands::Update { document, title } => {
            println!("Updating document: {}", document);
            if let Some(t) = title {
                println!("  New title: {}", t);
            }
            // TODO: Implement update logic
        }
        Commands::Search { query, title_only } => {
            if *title_only {
                println!("Searching titles for: {}", query);
            } else {
                println!("Searching all content for: {}", query);
            }
            // TODO: Implement search logic
        }
    }
}
