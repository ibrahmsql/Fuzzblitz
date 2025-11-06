#![allow(dead_code)]
pub struct SubdomainEnumerator;

impl SubdomainEnumerator {
    pub fn generate_subdomains(base_domain: &str, wordlist: &[String]) -> Vec<String> {
        wordlist.iter()
            .map(|word| format!("{}.{}", word, base_domain))
            .collect()
    }
    
    pub fn generate_with_variations(base_domain: &str, wordlist: &[String]) -> Vec<String> {
        let mut subdomains = Vec::new();
        
        for word in wordlist {
            subdomains.push(format!("{}.{}", word, base_domain));
            subdomains.push(format!("{}-{}", word, base_domain));
            subdomains.push(format!("{}_{}", word, base_domain));
        }
        
        subdomains
    }
}
