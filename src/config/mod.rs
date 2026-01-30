use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Configuration file name for mdlibs library
pub const CONFIG_FILE_NAME: &str = ".mdlibs.toml";

/// Default directory for templates
pub const TEMPLATES_DIR: &str = "templates";

/// Default directory for documents
pub const DOCS_DIR: &str = "docs";

/// Library configuration
#[derive(Debug, Clone)]
pub struct LibraryConfig {
    pub name: String,
    #[allow(dead_code)]
    pub path: PathBuf,
    pub version: String,
}

impl Default for LibraryConfig {
    fn default() -> Self {
        Self {
            name: String::from("mdlibs"),
            path: PathBuf::from("."),
            version: String::from("0.1.0"),
        }
    }
}

impl LibraryConfig {
    /// Create a new library configuration
    pub fn new(name: &str, path: PathBuf) -> Self {
        Self {
            name: name.to_string(),
            path,
            version: String::from("0.1.0"),
        }
    }

    /// Generate the config file content as TOML format
    pub fn to_toml(&self) -> String {
        format!(
            r#"# mdlibs configuration file
[library]
name = "{}"
version = "{}"
"#,
            self.name, self.version
        )
    }

    /// Load configuration from a path
    #[allow(dead_code)]
    pub fn load(path: &Path) -> io::Result<Self> {
        let config_path = path.join(CONFIG_FILE_NAME);
        if !config_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Configuration file not found. Run 'mdlibs init' first.",
            ));
        }

        let content = fs::read_to_string(&config_path)?;
        Self::parse_toml(&content, path)
    }

    /// Parse TOML content (simple parser for our format)
    fn parse_toml(content: &str, path: &Path) -> io::Result<Self> {
        let mut name = String::from("mdlibs");
        let mut version = String::from("0.1.0");

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("name") {
                if let Some(value) = Self::extract_toml_value(line) {
                    name = value;
                }
            } else if line.starts_with("version") {
                if let Some(value) = Self::extract_toml_value(line) {
                    version = value;
                }
            }
        }

        Ok(Self {
            name,
            path: path.to_path_buf(),
            version,
        })
    }

    /// Extract value from a TOML key-value line
    fn extract_toml_value(line: &str) -> Option<String> {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() == 2 {
            let value = parts[1].trim().trim_matches('"');
            Some(value.to_string())
        } else {
            None
        }
    }

    /// Find library root by searching for config file
    pub fn find_library_root(start_path: &Path) -> Option<PathBuf> {
        let mut current = start_path.to_path_buf();
        loop {
            let config_path = current.join(CONFIG_FILE_NAME);
            if config_path.exists() {
                return Some(current);
            }
            if !current.pop() {
                break;
            }
        }
        None
    }
}

/// Plugin trait for future extensibility
#[allow(dead_code)]
pub trait Plugin {
    /// Get the plugin name
    fn name(&self) -> &str;

    /// Get the plugin version
    fn version(&self) -> &str;

    /// Initialize the plugin
    fn init(&self) -> io::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_config_default() {
        let config = LibraryConfig::default();
        assert_eq!(config.name, "mdlibs");
        assert_eq!(config.version, "0.1.0");
    }

    #[test]
    fn test_library_config_new() {
        let config = LibraryConfig::new("my-lib", PathBuf::from("/test"));
        assert_eq!(config.name, "my-lib");
        assert_eq!(config.path, PathBuf::from("/test"));
    }

    #[test]
    fn test_to_toml() {
        let config = LibraryConfig::new("test-lib", PathBuf::from("."));
        let toml = config.to_toml();
        assert!(toml.contains("name = \"test-lib\""));
        assert!(toml.contains("version = \"0.1.0\""));
    }

    #[test]
    fn test_extract_toml_value() {
        assert_eq!(
            LibraryConfig::extract_toml_value("name = \"test\""),
            Some("test".to_string())
        );
        assert_eq!(
            LibraryConfig::extract_toml_value("version = \"1.0.0\""),
            Some("1.0.0".to_string())
        );
    }

    #[test]
    fn test_parse_toml() {
        let content = r#"
[library]
name = "my-library"
version = "2.0.0"
"#;
        let config = LibraryConfig::parse_toml(content, Path::new(".")).unwrap();
        assert_eq!(config.name, "my-library");
        assert_eq!(config.version, "2.0.0");
    }
}
