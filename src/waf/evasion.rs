use std::time::Duration;
use tokio::time::sleep;
use rand::Rng;

pub struct WafEvasion;

impl WafEvasion {
    pub fn random_user_agents() -> Vec<String> {
        vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/119.0".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/119.0".to_string(),
        ]
    }
    
    pub fn get_random_user_agent() -> String {
        let agents = Self::random_user_agents();
        let mut rng = rand::thread_rng();
        agents[rng.gen_range(0..agents.len())].clone()
    }
    
    pub async fn random_delay(min_ms: u64, max_ms: u64) {
        let mut rng = rand::thread_rng();
        let delay = rng.gen_range(min_ms..=max_ms);
        sleep(Duration::from_millis(delay)).await;
    }
    
    pub fn header_order_variations() -> Vec<Vec<&'static str>> {
        vec![
            vec!["User-Agent", "Accept", "Accept-Language", "Accept-Encoding", "Connection"],
            vec!["Accept", "User-Agent", "Accept-Encoding", "Accept-Language", "Connection"],
            vec!["Connection", "Accept-Encoding", "Accept-Language", "Accept", "User-Agent"],
        ]
    }
    
    pub fn case_variations(payload: &str) -> Vec<String> {
        let mut variations = Vec::new();
        
        variations.push(payload.to_lowercase());
        variations.push(payload.to_uppercase());
        
        let mut mixed = String::new();
        for (i, c) in payload.chars().enumerate() {
            if i % 2 == 0 {
                mixed.push_str(&c.to_uppercase().to_string());
            } else {
                mixed.push_str(&c.to_lowercase().to_string());
            }
        }
        variations.push(mixed);
        
        variations
    }
    
    pub fn ip_rotation_headers(ips: &[&str]) -> Vec<Vec<(&'static str, String)>> {
        let mut all_headers = Vec::new();
        
        for ip in ips {
            let headers = vec![
                ("X-Forwarded-For", ip.to_string()),
                ("X-Real-IP", ip.to_string()),
                ("X-Originating-IP", ip.to_string()),
                ("X-Remote-IP", ip.to_string()),
                ("X-Client-IP", ip.to_string()),
            ];
            all_headers.push(headers);
        }
        
        all_headers
    }
    
    pub fn fragment_payload(payload: &str, chunk_size: usize) -> Vec<String> {
        payload.chars()
            .collect::<Vec<char>>()
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().collect())
            .collect()
    }
}
