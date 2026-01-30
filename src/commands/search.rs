use std::fs;
use std::io;
use std::path::Path;

use crate::config::{LibraryConfig, DOCS_DIR, TEMPLATES_DIR};

/// Search result entry
#[derive(Debug)]
pub struct SearchResult {
    pub path: String,
    pub title: String,
    pub matches: Vec<SearchMatch>,
}

/// A single match within a document
#[derive(Debug)]
pub struct SearchMatch {
    pub line_number: usize,
    pub line_content: String,
}

/// Search through markdown documents
pub fn run(query: &str, title_only: bool) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;

    // Try to find library root
    let lib_root =
        LibraryConfig::find_library_root(&current_dir).unwrap_or_else(|| current_dir.clone());

    let results = search_documents(&lib_root, query, title_only)?;

    if results.is_empty() {
        println!("No results found for: {}", query);
        return Ok(());
    }

    println!("Found {} result(s) for '{}':\n", results.len(), query);

    for result in results {
        println!("📄 {} ({})", result.title, result.path);
        if !title_only {
            for m in &result.matches {
                let preview = truncate_line(&m.line_content, 60);
                println!("   Line {}: {}", m.line_number, preview);
            }
        }
        println!();
    }

    Ok(())
}

/// Search documents in the library
fn search_documents(
    lib_root: &Path,
    query: &str,
    title_only: bool,
) -> io::Result<Vec<SearchResult>> {
    let mut results = Vec::new();
    let query_lower = query.to_lowercase();

    // Collect all markdown files
    let mut files = Vec::new();
    collect_markdown_files(&lib_root.join(DOCS_DIR), &mut files)?;
    collect_markdown_files(&lib_root.join(TEMPLATES_DIR), &mut files)?;
    collect_markdown_files_single(lib_root, &mut files)?;

    for file_path in files {
        if let Some(result) = search_file(&file_path, lib_root, &query_lower, title_only)? {
            results.push(result);
        }
    }

    Ok(results)
}

/// Collect markdown files from a directory recursively
fn collect_markdown_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> io::Result<()> {
    if !dir.exists() || !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_markdown_files(&path, files)?;
        } else if is_markdown_file(&path) {
            files.push(path);
        }
    }

    Ok(())
}

/// Collect markdown files from a single directory (non-recursive)
fn collect_markdown_files_single(
    dir: &Path,
    files: &mut Vec<std::path::PathBuf>,
) -> io::Result<()> {
    if !dir.exists() || !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && is_markdown_file(&path) {
            files.push(path);
        }
    }

    Ok(())
}

/// Check if a path is a markdown file
fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .map(|ext| ext == "md" || ext == "markdown")
        .unwrap_or(false)
}

/// Search within a single file
fn search_file(
    path: &Path,
    lib_root: &Path,
    query: &str,
    title_only: bool,
) -> io::Result<Option<SearchResult>> {
    let content = fs::read_to_string(path)?;
    let title = extract_title(&content).unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string()
    });

    let relative_path = path
        .strip_prefix(lib_root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| path.to_string_lossy().to_string());

    if title_only {
        // Only search in title
        if title.to_lowercase().contains(query) {
            return Ok(Some(SearchResult {
                path: relative_path,
                title,
                matches: Vec::new(),
            }));
        }
    } else {
        // Search in full content
        let mut matches = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(query) {
                matches.push(SearchMatch {
                    line_number: line_num + 1,
                    line_content: line.to_string(),
                });
            }
        }

        if !matches.is_empty() {
            return Ok(Some(SearchResult {
                path: relative_path,
                title,
                matches,
            }));
        }
    }

    Ok(None)
}

/// Extract the title from markdown content
fn extract_title(content: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if let Some(title_text) = line.strip_prefix("# ") {
            return Some(title_text.trim().to_string());
        }
    }
    None
}

/// Truncate a line for display
fn truncate_line(line: &str, max_len: usize) -> String {
    let trimmed = line.trim();
    if trimmed.len() <= max_len {
        trimmed.to_string()
    } else {
        format!("{}...", &trimmed[..max_len])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_is_markdown_file() {
        assert!(is_markdown_file(Path::new("test.md")));
        assert!(is_markdown_file(Path::new("test.markdown")));
        assert!(!is_markdown_file(Path::new("test.txt")));
    }

    #[test]
    fn test_extract_title() {
        let content = "# Test Title\n\nContent";
        assert_eq!(extract_title(content), Some("Test Title".to_string()));
    }

    #[test]
    fn test_truncate_line() {
        assert_eq!(truncate_line("short", 10), "short");
        assert_eq!(truncate_line("this is a longer line", 10), "this is a ...");
    }

    #[test]
    fn test_search_file() {
        let temp_dir = env::temp_dir().join("mdlibs_test_search");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let test_file = temp_dir.join("test.md");
        fs::write(
            &test_file,
            "# Test Document\n\nThis contains the word rust.",
        )
        .unwrap();

        let result = search_file(&test_file, &temp_dir, "rust", false).unwrap();
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.title, "Test Document");
        assert_eq!(result.matches.len(), 1);

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_search_title_only() {
        let temp_dir = env::temp_dir().join("mdlibs_test_search_title");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let test_file = temp_dir.join("test.md");
        fs::write(&test_file, "# Rust Guide\n\nThis is about programming.").unwrap();

        // Should match title
        let result = search_file(&test_file, &temp_dir, "rust", true).unwrap();
        assert!(result.is_some());

        // Should not match - query not in title
        let result = search_file(&test_file, &temp_dir, "programming", true).unwrap();
        assert!(result.is_none());

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
