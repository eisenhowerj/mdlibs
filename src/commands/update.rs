use std::fs;
use std::io;
use std::path::Path;

use crate::config::{LibraryConfig, DOCS_DIR, TEMPLATES_DIR};
use crate::utils::extract_title_from_content;

/// Update metadata of a markdown document
pub fn run(document: &str, title: Option<&str>) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;

    // Try to find library root
    let lib_root =
        LibraryConfig::find_library_root(&current_dir).unwrap_or_else(|| current_dir.clone());

    // Find the document
    let doc_path = find_document(&lib_root, document)?;

    if title.is_none() {
        // Just display current document info
        display_document_info(&doc_path)?;
        return Ok(());
    }

    // Read current content
    let content = fs::read_to_string(&doc_path)?;

    // Update title if provided
    if let Some(new_title) = title {
        let updated_content = update_document_title(&content, new_title);
        fs::write(&doc_path, updated_content)?;
        println!("Updated document: {}", doc_path.display());
        println!("  New title: {}", new_title);
    }

    Ok(())
}

/// Find a document by name or path
fn find_document(lib_root: &Path, document: &str) -> io::Result<std::path::PathBuf> {
    // Try direct path first
    let direct_path = lib_root.join(document);
    if direct_path.exists() {
        return Ok(direct_path);
    }

    // Try in docs directory
    let docs_path = lib_root.join(DOCS_DIR).join(document);
    if docs_path.exists() {
        return Ok(docs_path);
    }

    // Try in templates directory
    let templates_path = lib_root.join(TEMPLATES_DIR).join(document);
    if templates_path.exists() {
        return Ok(templates_path);
    }

    // Try adding .md extension
    let with_ext = format!("{}.md", document);

    let direct_with_ext = lib_root.join(&with_ext);
    if direct_with_ext.exists() {
        return Ok(direct_with_ext);
    }

    let docs_with_ext = lib_root.join(DOCS_DIR).join(&with_ext);
    if docs_with_ext.exists() {
        return Ok(docs_with_ext);
    }

    let templates_with_ext = lib_root.join(TEMPLATES_DIR).join(&with_ext);
    if templates_with_ext.exists() {
        return Ok(templates_with_ext);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("Document not found: {}", document),
    ))
}

/// Display information about a document
fn display_document_info(path: &Path) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    let title = extract_title_from_content(&content).unwrap_or_else(|| "Untitled".to_string());
    let word_count = count_words(&content);
    let line_count = content.lines().count();

    println!("Document: {}", path.display());
    println!("  Title: {}", title);
    println!("  Lines: {}", line_count);
    println!("  Words: {}", word_count);

    Ok(())
}

/// Update the title (first H1 heading) in a markdown document
fn update_document_title(content: &str, new_title: &str) -> String {
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    let mut found_title = false;

    for line in &mut lines {
        if line.trim().starts_with("# ") {
            *line = format!("# {}", new_title);
            found_title = true;
            break;
        }
    }

    if !found_title {
        // Prepend title if none exists, followed by empty line
        lines.insert(0, String::new());
        lines.insert(0, format!("# {}", new_title));
    }

    let mut result = lines.join("\n");
    result.push('\n');
    result
}

/// Count words in content
fn count_words(content: &str) -> usize {
    content.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_document_title_existing() {
        let content = "# Old Title\n\nSome content here.";
        let result = update_document_title(content, "New Title");
        assert!(result.contains("# New Title"));
        assert!(!result.contains("Old Title"));
    }

    #[test]
    fn test_update_document_title_new() {
        let content = "Some content without title.";
        let result = update_document_title(content, "New Title");
        assert!(result.starts_with("# New Title"));
        // Verify the new title is followed by an empty line separator
        assert!(result.contains("# New Title\n\n"));
    }

    #[test]
    fn test_count_words() {
        let content = "One two three four five";
        assert_eq!(count_words(content), 5);
    }
}
