#![allow(dead_code)]
use std::collections::HashMap;
use super::wordlist::Wordlist;

#[derive(Debug, Clone, Copy)]
pub enum FuzzMode {
    Clusterbomb, // All combinations (cartesian product)
    Pitchfork,   // Parallel iteration
    Sniper,      // One wordlist, iterate through each position
}

impl FuzzMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pitchfork" => FuzzMode::Pitchfork,
            "sniper" => FuzzMode::Sniper,
            _ => FuzzMode::Clusterbomb,
        }
    }
}

#[derive(Clone)]
pub struct PayloadGenerator {
    pub wordlists: Vec<Wordlist>,
    pub mode: FuzzMode,
    current_indices: Vec<usize>,
    done: bool,
}

impl PayloadGenerator {
    pub fn new(wordlists: Vec<Wordlist>, mode: FuzzMode) -> Self {
        let current_indices = vec![0; wordlists.len()];
        let done = wordlists.is_empty() || wordlists.iter().any(|w| w.words.is_empty());
        
        Self {
            wordlists,
            mode,
            current_indices,
            done,
        }
    }
    
    pub fn total_requests(&self) -> usize {
        if self.wordlists.is_empty() {
            return 0;
        }
        
        match self.mode {
            FuzzMode::Clusterbomb => {
                self.wordlists.iter()
                    .map(|w| w.words.len())
                    .product()
            },
            FuzzMode::Pitchfork => {
                self.wordlists.iter()
                    .map(|w| w.words.len())
                    .min()
                    .unwrap_or(0)
            },
            FuzzMode::Sniper => {
                self.wordlists.iter()
                    .map(|w| w.words.len())
                    .sum()
            },
        }
    }
}

impl Iterator for PayloadGenerator {
    type Item = HashMap<String, String>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        
        match self.mode {
            FuzzMode::Clusterbomb => self.next_clusterbomb(),
            FuzzMode::Pitchfork => self.next_pitchfork(),
            FuzzMode::Sniper => self.next_sniper(),
        }
    }
}

impl PayloadGenerator {
    fn next_clusterbomb(&mut self) -> Option<HashMap<String, String>> {
        let mut result = HashMap::new();
        
        // Build current combination
        for (i, wordlist) in self.wordlists.iter().enumerate() {
            let word = &wordlist.words[self.current_indices[i]];
            result.insert(wordlist.keyword.clone(), word.clone());
        }
        
        // Increment indices (like odometer)
        let mut carry = true;
        for i in (0..self.current_indices.len()).rev() {
            if carry {
                self.current_indices[i] += 1;
                if self.current_indices[i] >= self.wordlists[i].words.len() {
                    self.current_indices[i] = 0;
                } else {
                    carry = false;
                }
            }
        }
        
        if carry {
            self.done = true;
        }
        
        Some(result)
    }
    
    fn next_pitchfork(&mut self) -> Option<HashMap<String, String>> {
        let mut result = HashMap::new();
        
        // Take one from each wordlist in parallel
        for (i, wordlist) in self.wordlists.iter().enumerate() {
            if self.current_indices[i] >= wordlist.words.len() {
                self.done = true;
                return None;
            }
            let word = &wordlist.words[self.current_indices[i]];
            result.insert(wordlist.keyword.clone(), word.clone());
        }
        
        // Increment all indices
        for idx in self.current_indices.iter_mut() {
            *idx += 1;
        }
        
        Some(result)
    }
    
    fn next_sniper(&mut self) -> Option<HashMap<String, String>> {
        // Sniper mode: one wordlist, iterate through each position
        if self.wordlists.is_empty() {
            self.done = true;
            return None;
        }
        
        let current_wordlist_idx = self.current_indices[0];
        let current_word_idx = self.current_indices[1];
        
        if current_wordlist_idx >= self.wordlists.len() {
            self.done = true;
            return None;
        }
        
        let wordlist = &self.wordlists[current_wordlist_idx];
        
        if current_word_idx >= wordlist.words.len() {
            // Move to next wordlist
            self.current_indices[0] += 1;
            self.current_indices[1] = 0;
            return self.next_sniper();
        }
        
        let mut result = HashMap::new();
        result.insert(wordlist.keyword.clone(), wordlist.words[current_word_idx].clone());
        
        self.current_indices[1] += 1;
        
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clusterbomb() {
        let wordlists = vec![
            Wordlist {
                keyword: "USER".to_string(),
                words: vec!["admin".to_string(), "root".to_string()],
            },
            Wordlist {
                keyword: "PASS".to_string(),
                words: vec!["123".to_string(), "456".to_string()],
            },
        ];
        
        let mut gen = PayloadGenerator::new(wordlists, FuzzMode::Clusterbomb);
        assert_eq!(gen.total_requests(), 4);
        
        let combinations: Vec<_> = gen.collect();
        assert_eq!(combinations.len(), 4);
    }

    #[test]
    fn test_pitchfork() {
        let wordlists = vec![
            Wordlist {
                keyword: "USER".to_string(),
                words: vec!["admin".to_string(), "root".to_string(), "guest".to_string()],
            },
            Wordlist {
                keyword: "PASS".to_string(),
                words: vec!["123".to_string(), "456".to_string()],
            },
        ];
        
        let mut gen = PayloadGenerator::new(wordlists, FuzzMode::Pitchfork);
        assert_eq!(gen.total_requests(), 2); // Min of both
        
        let combinations: Vec<_> = gen.collect();
        assert_eq!(combinations.len(), 2);
    }
}
