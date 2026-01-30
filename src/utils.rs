use std::fs;
use std::path::Path;

/// Check if a path is a markdown file
pub fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext.to_string_lossy().to_lowercase())
        .map(|ext| ext == "md" || ext == "markdown")
        .unwrap_or(false)
}

/// Extract the title (first H1 heading) from markdown content
pub fn extract_title_from_content(content: &str) -> Option<String> {
    for line in content.lines() {
        let line = line.trim();
        if let Some(title_text) = line.strip_prefix("# ") {
            let trimmed_title = title_text.trim();
            // Return None if the title is empty or whitespace-only
            if !trimmed_title.is_empty() {
                return Some(trimmed_title.to_string());
            }
        }
    }
    None
}

/// Extract the title from a markdown file
pub fn extract_title_from_file(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    extract_title_from_content(&content)
}

/// Truncate a string safely for display (UTF-8 safe)
pub fn truncate_display(s: &str, max_len: usize) -> String {
    let trimmed = s.trim();
    if trimmed.chars().count() <= max_len {
        trimmed.to_string()
    } else {
        let truncated: String = trimmed.chars().take(max_len).collect();
        format!("{}...", truncated)
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
        assert!(is_markdown_file(Path::new("test.MD")));
        assert!(!is_markdown_file(Path::new("test.txt")));
        assert!(!is_markdown_file(Path::new("test")));
    }

    #[test]
    fn test_extract_title_from_content() {
        let content = "# My Title\n\nContent here";
        assert_eq!(
            extract_title_from_content(content),
            Some("My Title".to_string())
        );
    }

    #[test]
    fn test_extract_title_from_content_none() {
        let content = "No title here";
        assert_eq!(extract_title_from_content(content), None);
    }

    #[test]
    fn test_extract_title_from_content_empty() {
        // Empty title should return None
        let content = "# \n\nContent";
        assert_eq!(extract_title_from_content(content), None);
    }

    #[test]
    fn test_extract_title_from_file() {
        let temp_dir = env::temp_dir().join("mdlibs_test_utils");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let test_file = temp_dir.join("test.md");
        fs::write(&test_file, "# File Title\n\nContent").unwrap();

        let title = extract_title_from_file(&test_file);
        assert_eq!(title, Some("File Title".to_string()));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_truncate_display() {
        assert_eq!(truncate_display("short", 10), "short");
        assert_eq!(
            truncate_display("this is a longer line", 10),
            "this is a ..."
        );
    }

    #[test]
    fn test_truncate_display_unicode() {
        // Test with unicode characters
        let unicode_str = "日本語テスト";
        assert_eq!(truncate_display(unicode_str, 3), "日本語...");
    }
}
