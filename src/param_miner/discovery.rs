#![allow(dead_code)]
pub struct ParamDiscovery;

impl ParamDiscovery {
    pub fn mine_params(url: &str, wordlist: &[String]) -> Vec<String> {
        wordlist.iter()
            .map(|param| format!("{}?{}=test", url, param))
            .collect()
    }
    
    pub fn test_methods() -> Vec<String> {
        vec!["GET", "POST"].iter().map(|s| s.to_string()).collect()
    }
}
