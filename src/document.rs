use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Frontmatter {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub doc_type: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub resource: Option<String>,
    pub tags: Option<Vec<String>>,
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Document {
    pub path: PathBuf,
    pub frontmatter: Frontmatter,
    #[allow(dead_code)]
    pub body: String,
    pub md_links: Vec<String>,
}

impl Document {
    pub fn from_path(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Cannot read {}: {}", path.display(), e))?;
        Self::from_str(&content, path.to_path_buf())
    }

    pub fn from_str(content: &str, path: PathBuf) -> anyhow::Result<Self> {
        let parsed = fronma::parser::parse::<Frontmatter>(content)
            .map_err(|e| anyhow::anyhow!("Frontmatter error in {}: {:?}", path.display(), e))?;
        let md_links = extract_md_links(parsed.body);
        Ok(Self {
            path,
            frontmatter: parsed.headers,
            body: parsed.body.to_string(),
            md_links,
        })
    }

    pub fn filename(&self) -> &str {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
    }

    pub fn is_index(&self) -> bool {
        self.filename() == "index.md"
    }

    pub fn is_log(&self) -> bool {
        self.filename() == "log.md"
    }
}

fn extract_md_links(text: &str) -> Vec<String> {
    let re = regex::Regex::new(r"\[[^\]]*\]\(([^)]+\.md(?:#[^)]*)?)\)").unwrap();
    re.captures_iter(text)
        .map(|cap| cap[1].to_string())
        .collect()
}
