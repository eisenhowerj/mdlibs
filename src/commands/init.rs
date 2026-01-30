use std::fs;
use std::io;
use std::path::Path;

use crate::config::{LibraryConfig, CONFIG_FILE_NAME, DOCS_DIR, TEMPLATES_DIR};

/// Initialize a new markdown library at the given path
pub fn run(path: &str) -> io::Result<()> {
    let lib_path = Path::new(path);

    // Check if already initialized
    let config_path = lib_path.join(CONFIG_FILE_NAME);
    if config_path.exists() {
        println!("Library already initialized at: {}", path);
        return Ok(());
    }

    // Create the directory if it doesn't exist
    if !lib_path.exists() {
        fs::create_dir_all(lib_path)?;
        println!("Created directory: {}", path);
    }

    // Create configuration file
    let lib_name = lib_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("mdlibs");

    let config = LibraryConfig::new(lib_name, lib_path.to_path_buf());
    fs::write(&config_path, config.to_toml())?;
    println!("Created configuration file: {}", config_path.display());

    // Create default directories
    let templates_path = lib_path.join(TEMPLATES_DIR);
    if !templates_path.exists() {
        fs::create_dir_all(&templates_path)?;
        println!("Created directory: {}", templates_path.display());
    }

    let docs_path = lib_path.join(DOCS_DIR);
    if !docs_path.exists() {
        fs::create_dir_all(&docs_path)?;
        println!("Created directory: {}", docs_path.display());
    }

    // Create a sample README.md
    let readme_path = docs_path.join("README.md");
    if !readme_path.exists() {
        let readme_content = format!(
            r#"# {}

Welcome to your markdown library!

## Getting Started

This library was initialized with mdlibs. You can:

- Add markdown documents to the `docs/` directory
- Create templates in the `templates/` directory
- Use `mdlibs list` to see all documents
- Use `mdlibs search <query>` to search documents

## Structure

- `docs/` - Your markdown documents
- `templates/` - Reusable document templates
- `.mdlibs.toml` - Library configuration
"#,
            lib_name
        );
        fs::write(&readme_path, readme_content)?;
        println!("Created sample document: {}", readme_path.display());
    }

    println!("\nMarkdown library initialized successfully at: {}", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_init_creates_structure() {
        let temp_dir = env::temp_dir().join("mdlibs_test_init");
        let _ = fs::remove_dir_all(&temp_dir);

        let path = temp_dir.to_str().unwrap();
        run(path).unwrap();

        assert!(temp_dir.join(CONFIG_FILE_NAME).exists());
        assert!(temp_dir.join(TEMPLATES_DIR).exists());
        assert!(temp_dir.join(DOCS_DIR).exists());
        assert!(temp_dir.join(DOCS_DIR).join("README.md").exists());

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_init_idempotent() {
        let temp_dir = env::temp_dir().join("mdlibs_test_init_idempotent");
        let _ = fs::remove_dir_all(&temp_dir);

        let path = temp_dir.to_str().unwrap();
        run(path).unwrap();
        run(path).unwrap(); // Should not error

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
