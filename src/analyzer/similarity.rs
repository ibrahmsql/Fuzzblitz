/// Calculate similarity between responses
pub struct SimilarityAnalyzer;

impl SimilarityAnalyzer {
    /// Calculate Levenshtein distance
    pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
        let len1 = s1.chars().count();
        let len2 = s2.chars().count();
        
        if len1 == 0 { return len2; }
        if len2 == 0 { return len1; }
        
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }
        
        let s1_chars: Vec<char> = s1.chars().collect();
        let s2_chars: Vec<char> = s2.chars().collect();
        
        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = std::cmp::min(
                    std::cmp::min(
                        matrix[i - 1][j] + 1,      // deletion
                        matrix[i][j - 1] + 1       // insertion
                    ),
                    matrix[i - 1][j - 1] + cost    // substitution
                );
            }
        }
        
        matrix[len1][len2]
    }
    
    /// Calculate similarity ratio (0.0 to 1.0)
    pub fn similarity_ratio(s1: &str, s2: &str) -> f64 {
        let distance = Self::levenshtein_distance(s1, s2);
        let max_len = std::cmp::max(s1.len(), s2.len());
        
        if max_len == 0 {
            return 1.0;
        }
        
        1.0 - (distance as f64 / max_len as f64)
    }
    
    /// Check if two strings are similar (above threshold)
    pub fn is_similar(s1: &str, s2: &str, threshold: f64) -> bool {
        Self::similarity_ratio(s1, s2) >= threshold
    }
    
    /// Calculate Jaccard similarity
    pub fn jaccard_similarity(s1: &str, s2: &str) -> f64 {
        let set1: std::collections::HashSet<char> = s1.chars().collect();
        let set2: std::collections::HashSet<char> = s2.chars().collect();
        
        let intersection = set1.intersection(&set2).count();
        let union = set1.union(&set2).count();
        
        if union == 0 {
            return 0.0;
        }
        
        intersection as f64 / union as f64
    }
    
    /// Find most similar response in a list
    pub fn find_most_similar<'a>(target: &str, candidates: &'a [String]) -> Option<&'a String> {
        candidates.iter()
            .max_by(|a, b| {
                let sim_a = Self::similarity_ratio(target, a);
                let sim_b = Self::similarity_ratio(target, b);
                sim_a.partial_cmp(&sim_b).unwrap()
            })
    }
    
    /// Group similar responses
    pub fn group_similar(responses: &[String], threshold: f64) -> Vec<Vec<String>> {
        let mut groups: Vec<Vec<String>> = Vec::new();
        
        for response in responses {
            let mut found_group = false;
            
            for group in &mut groups {
                if let Some(first) = group.first() {
                    if Self::is_similar(response, first, threshold) {
                        group.push(response.clone());
                        found_group = true;
                        break;
                    }
                }
            }
            
            if !found_group {
                groups.push(vec![response.clone()]);
            }
        }
        
        groups
    }
}
