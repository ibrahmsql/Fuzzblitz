use std::time::Duration;

pub struct StealthMode;

impl StealthMode {
    pub fn calculate_adaptive_delay(error_count: usize, base_delay_ms: u64) -> Duration {
        let multiplier = if error_count > 10 {
            5
        } else if error_count > 5 {
            3
        } else if error_count > 2 {
            2
        } else {
            1
        };
        
        Duration::from_millis(base_delay_ms * multiplier)
    }
    
    pub fn should_pause(consecutive_errors: usize, threshold: usize) -> bool {
        consecutive_errors >= threshold
    }
    
    pub fn pause_duration(consecutive_errors: usize) -> Duration {
        let seconds = match consecutive_errors {
            0..=5 => 5,
            6..=10 => 30,
            11..=20 => 60,
            _ => 120,
        };
        
        Duration::from_secs(seconds)
    }
    
    pub fn legitimate_browser_headers() -> Vec<(&'static str, &'static str)> {
        vec![
            ("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8"),
            ("Accept-Language", "en-US,en;q=0.9"),
            ("Accept-Encoding", "gzip, deflate, br"),
            ("DNT", "1"),
            ("Connection", "keep-alive"),
            ("Upgrade-Insecure-Requests", "1"),
            ("Sec-Fetch-Dest", "document"),
            ("Sec-Fetch-Mode", "navigate"),
            ("Sec-Fetch-Site", "none"),
            ("Cache-Control", "max-age=0"),
        ]
    }
    
    pub fn is_suspicious_response(status_code: u16, body_size: usize) -> bool {
        status_code == 403 || 
        status_code == 429 || 
        status_code == 503 ||
        body_size < 100
    }
}
