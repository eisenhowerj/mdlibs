use std::fs;
use std::io;
use std::path::Path;

use crate::config::{LibraryConfig, DOCS_DIR, TEMPLATES_DIR};

/// Document entry representing a markdown file
#[derive(Debug)]
pub struct DocumentEntry {
    pub path: String,
    pub title: String,
    pub doc_type: DocumentType,
}

/// Type of document
#[derive(Debug, PartialEq)]
pub enum DocumentType {
    Document,
    Template,
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentType::Document => write!(f, "doc"),
            DocumentType::Template => write!(f, "template"),
        }
    }
}

/// List markdown documents in the library
pub fn run(filter: Option<&str>) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;

    // Try to find library root
    let lib_root =
        LibraryConfig::find_library_root(&current_dir).unwrap_or_else(|| current_dir.clone());

    let documents = collect_documents(&lib_root)?;

    if documents.is_empty() {
        println!("No markdown documents found.");
        println!("Hint: Run 'mdlibs init' to initialize a library, or add .md files to the docs/ directory.");
        return Ok(());
    }

    // Apply filter if provided
    let filtered_docs: Vec<&DocumentEntry> = if let Some(f) = filter {
        let filter_lower = f.to_lowercase();
        documents
            .iter()
            .filter(|d| {
                d.title.to_lowercase().contains(&filter_lower)
                    || d.path.to_lowercase().contains(&filter_lower)
            })
            .collect()
    } else {
        documents.iter().collect()
    };

    if filtered_docs.is_empty() {
        println!("No documents match filter: {}", filter.unwrap_or(""));
        return Ok(());
    }

    println!("Found {} document(s):\n", filtered_docs.len());
    println!("{:<10} {:<40} PATH", "TYPE", "TITLE");
    println!("{}", "-".repeat(70));

    for doc in filtered_docs {
        println!("{:<10} {:<40} {}", doc.doc_type, doc.title, doc.path);
    }

    Ok(())
}

/// Collect all markdown documents from the library
fn collect_documents(lib_root: &Path) -> io::Result<Vec<DocumentEntry>> {
    let mut documents = Vec::new();

    // Scan docs directory
    let docs_path = lib_root.join(DOCS_DIR);
    if docs_path.exists() {
        scan_directory(&docs_path, lib_root, DocumentType::Document, &mut documents)?;
    }

    // Scan templates directory
    let templates_path = lib_root.join(TEMPLATES_DIR);
    if templates_path.exists() {
        scan_directory(
            &templates_path,
            lib_root,
            DocumentType::Template,
            &mut documents,
        )?;
    }

    // Also scan root for any markdown files
    scan_single_directory(lib_root, lib_root, DocumentType::Document, &mut documents)?;

    Ok(documents)
}

/// Scan a directory recursively for markdown files
fn scan_directory(
    dir: &Path,
    lib_root: &Path,
    doc_type: DocumentType,
    documents: &mut Vec<DocumentEntry>,
) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let sub_type = if doc_type == DocumentType::Template {
                DocumentType::Template
            } else {
                DocumentType::Document
            };
            scan_directory(&path, lib_root, sub_type, documents)?;
        } else if is_markdown_file(&path) {
            if let Some(doc_entry) = create_document_entry(&path, lib_root, &doc_type) {
                documents.push(doc_entry);
            }
        }
    }

    Ok(())
}

/// Scan only a single directory (non-recursive) for markdown files
fn scan_single_directory(
    dir: &Path,
    lib_root: &Path,
    doc_type: DocumentType,
    documents: &mut Vec<DocumentEntry>,
) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && is_markdown_file(&path) {
            if let Some(doc_entry) = create_document_entry(&path, lib_root, &doc_type) {
                documents.push(doc_entry);
            }
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

/// Create a document entry from a file path
fn create_document_entry(
    path: &Path,
    lib_root: &Path,
    doc_type: &DocumentType,
) -> Option<DocumentEntry> {
    let relative_path = path.strip_prefix(lib_root).ok()?;
    let title = extract_title(path).unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string()
    });

    Some(DocumentEntry {
        path: relative_path.to_string_lossy().to_string(),
        title,
        doc_type: if *doc_type == DocumentType::Template {
            DocumentType::Template
        } else {
            DocumentType::Document
        },
    })
}

/// Extract title from markdown file (first H1 heading or filename)
fn extract_title(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;

    for line in content.lines() {
        let line = line.trim();
        if let Some(title_text) = line.strip_prefix("# ") {
            return Some(title_text.trim().to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_is_markdown_file() {
        assert!(is_markdown_file(Path::new("test.md")));
        assert!(is_markdown_file(Path::new("test.markdown")));
        assert!(is_markdown_file(Path::new("test.MD")));
        assert!(!is_markdown_file(Path::new("test.txt")));
        assert!(!is_markdown_file(Path::new("test")));
    }

    #[test]
    fn test_extract_title() {
        let temp_dir = env::temp_dir().join("mdlibs_test_list");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let test_file = temp_dir.join("test.md");
        fs::write(&test_file, "# My Title\n\nContent here").unwrap();

        let title = extract_title(&test_file);
        assert_eq!(title, Some("My Title".to_string()));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_document_type_display() {
        assert_eq!(format!("{}", DocumentType::Document), "doc");
        assert_eq!(format!("{}", DocumentType::Template), "template");
    }
}
