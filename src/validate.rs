use crate::document::Document;
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

pub enum ValidationResult {
    Pass(String),
    Warn(String, Vec<String>),
    Fail(String, String),
}

pub fn validate_path(path: &Path) -> Result<Vec<ValidationResult>> {
    let mut results = Vec::new();
    if path.is_file() {
        results.push(validate_file(path));
    } else {
        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
        {
            results.push(validate_file(entry.path()));
        }
    }
    Ok(results)
}

fn validate_file(path: &Path) -> ValidationResult {
    let label = path.display().to_string();

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return ValidationResult::Fail(label, format!("Cannot read: {e}")),
    };

    if !content.starts_with("---") {
        return ValidationResult::Fail(label, "No YAML frontmatter (file must start with ---)".into());
    }

    let doc = match Document::from_str(&content, path.to_path_buf()) {
        Ok(d) => d,
        Err(e) => return ValidationResult::Fail(label, format!("Parse error: {e}")),
    };

    if doc.frontmatter.doc_type.trim().is_empty() {
        return ValidationResult::Fail(label, "Required field `type` is empty".into());
    }

    let mut missing = Vec::new();
    if doc.frontmatter.title.is_none() {
        missing.push("title");
    }
    if doc.frontmatter.description.is_none() {
        missing.push("description");
    }
    if doc.frontmatter.timestamp.is_none() {
        missing.push("timestamp");
    }
    if doc.frontmatter.id.is_none() {
        missing.push("id (UUID)");
    }

    if missing.is_empty() {
        ValidationResult::Pass(label)
    } else {
        ValidationResult::Warn(label, missing.into_iter().map(String::from).collect())
    }
}

pub fn print_results(results: &[ValidationResult]) {
    let (mut pass, mut warn, mut fail) = (0u32, 0u32, 0u32);
    for r in results {
        match r {
            ValidationResult::Pass(p) => {
                println!("✅ PASS  {p}");
                pass += 1;
            }
            ValidationResult::Warn(p, missing) => {
                println!("⚠️  WARN  {p} — missing recommended: {}", missing.join(", "));
                warn += 1;
            }
            ValidationResult::Fail(p, reason) => {
                println!("❌ FAIL  {p} — {reason}");
                fail += 1;
            }
        }
    }
    println!("\nTotal: {pass} pass, {warn} warn, {fail} fail");
}
