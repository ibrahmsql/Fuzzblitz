#![allow(dead_code)]
use std::fs::{File, OpenOptions};
use std::io::Write;
use chrono::Local;
use super::result::FuzzResult;
use super::format::OutputFormat;

pub struct OutputWriter {
    format: OutputFormat,
    file: Option<File>,
}

impl OutputWriter {
    pub fn new(output_path: Option<String>, format: OutputFormat) -> Result<Self, std::io::Error> {
        let file = if let Some(path) = output_path {
            Some(OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)?)
        } else {
            None
        };
        
        Ok(Self { format, file })
    }
    
    pub fn write_header(&mut self) -> Result<(), std::io::Error> {
        if let Some(ref mut f) = self.file {
            match self.format {
                OutputFormat::Html => {
                    writeln!(f, "<!DOCTYPE html>")?;
                    writeln!(f, "<html><head><title>RustFuzz Results</title>")?;
                    writeln!(f, "<style>")?;
                    writeln!(f, "body {{ font-family: Arial, sans-serif; margin: 20px; }}")?;
                    writeln!(f, "table {{ border-collapse: collapse; width: 100%; }}")?;
                    writeln!(f, "th, td {{ border: 1px solid #ddd; padding: 8px; text-align: left; }}")?;
                    writeln!(f, "th {{ background-color: #4CAF50; color: white; }}")?;
                    writeln!(f, "tr:nth-child(even) {{ background-color: #f2f2f2; }}")?;
                    writeln!(f, ".status-2xx {{ color: green; }}")?;
                    writeln!(f, ".status-3xx {{ color: blue; }}")?;
                    writeln!(f, ".status-4xx {{ color: orange; }}")?;
                    writeln!(f, ".status-5xx {{ color: red; }}")?;
                    writeln!(f, "</style></head><body>")?;
                    writeln!(f, "<h1>RustFuzz Scan Results</h1>")?;
                    writeln!(f, "<p>Generated: {}</p>", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
                    writeln!(f, "<table>")?;
                    writeln!(f, "<tr><th>Keyword</th><th>URL</th><th>Status</th><th>Size</th><th>Lines</th><th>Words</th><th>Time (ms)</th></tr>")?;
                },
                OutputFormat::Csv => {
                    writeln!(f, "keyword,url,status_code,size,lines,words,response_time_ms,timestamp")?;
                },
                OutputFormat::Markdown => {
                    writeln!(f, "# RustFuzz Scan Results\n")?;
                    writeln!(f, "Generated: {}\n", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
                    writeln!(f, "| Keyword | URL | Status | Size | Lines | Words | Time (ms) |")?;
                    writeln!(f, "|---------|-----|--------|------|-------|-------|-----------|")?;
                },
                _ => {},
            }
        }
        Ok(())
    }
    
    pub fn write_result(&mut self, result: &FuzzResult) -> Result<(), std::io::Error> {
        if let Some(ref mut f) = self.file {
            match self.format {
                OutputFormat::Json => {
                    let json = serde_json::to_string(result)?;
                    writeln!(f, "{}", json)?;
                },
                OutputFormat::Csv => {
                    writeln!(
                        f,
                        "{},{},{},{},{},{},{},{}",
                        escape_csv(&result.fuzz_word),
                        escape_csv(&result.url),
                        result.status_code,
                        result.body_length,
                        result.lines,
                        result.words,
                        result.response_time_ms,
                        result.timestamp
                    )?;
                },
                OutputFormat::Html => {
                    let status_class = match result.status_code {
                        200..=299 => "status-2xx",
                        300..=399 => "status-3xx",
                        400..=499 => "status-4xx",
                        _ => "status-5xx",
                    };
                    writeln!(
                        f,
                        "<tr><td>{}</td><td>{}</td><td class=\"{}\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                        html_escape(&result.fuzz_word),
                        html_escape(&result.url),
                        status_class,
                        result.status_code,
                        result.body_length,
                        result.lines,
                        result.words,
                        result.response_time_ms
                    )?;
                },
                OutputFormat::Markdown => {
                    writeln!(
                        f,
                        "| {} | {} | {} | {} | {} | {} | {} |",
                        result.fuzz_word,
                        result.url,
                        result.status_code,
                        result.body_length,
                        result.lines,
                        result.words,
                        result.response_time_ms
                    )?;
                },
                OutputFormat::All => {
                    let json = serde_json::to_string(result)?;
                    writeln!(f, "{}", json)?;
                },
            }
            f.flush()?;
        }
        Ok(())
    }
    
    pub fn write_footer(&mut self) -> Result<(), std::io::Error> {
        if let Some(ref mut f) = self.file {
            match self.format {
                OutputFormat::Html => {
                    writeln!(f, "</table>")?;
                    writeln!(f, "</body></html>")?;
                },
                _ => {},
            }
            f.flush()?;
        }
        Ok(())
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
