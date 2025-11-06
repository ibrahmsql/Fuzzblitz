use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct Wordlist {
    pub keyword: String,
    pub words: Vec<String>,
}

impl Wordlist {
    pub fn from_file(path: &str, keyword: Option<String>) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut words = Vec::new();
        for line in reader.lines() {
            if let Ok(word) = line {
                let trimmed = word.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('#') {
                    words.push(trimmed.to_string());
                }
            }
        }
        
        let keyword = keyword.unwrap_or_else(|| "FUZZ".to_string());
        
        Ok(Self { keyword, words })
    }
    
    pub fn from_vec(words: Vec<String>, keyword: String) -> Self {
        Self { keyword, words }
    }
    
    pub fn len(&self) -> usize {
        self.words.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.words.is_empty()
    }
}

pub fn parse_wordlist_spec(spec: &str) -> (String, Option<String>) {
    // Format: "/path/to/wordlist:KEYWORD" or just "/path/to/wordlist"
    if let Some(pos) = spec.rfind(':') {
        let path = spec[..pos].to_string();
        let keyword = spec[pos + 1..].to_string();
        (path, Some(keyword))
    } else {
        (spec.to_string(), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_wordlist_spec() {
        let (path, keyword) = parse_wordlist_spec("/tmp/words.txt:FUZZ");
        assert_eq!(path, "/tmp/words.txt");
        assert_eq!(keyword, Some("FUZZ".to_string()));
        
        let (path, keyword) = parse_wordlist_spec("/tmp/words.txt");
        assert_eq!(path, "/tmp/words.txt");
        assert_eq!(keyword, None);
    }
}
