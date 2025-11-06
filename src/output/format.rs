#![allow(dead_code)]
/// Output format options for fuzzing results

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Csv,
    Html,
    Markdown,
    All,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "csv" => OutputFormat::Csv,
            "html" => OutputFormat::Html,
            "md" | "markdown" => OutputFormat::Markdown,
            "all" => OutputFormat::All,
            _ => OutputFormat::Json,
        }
    }
}
