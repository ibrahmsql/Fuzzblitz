#![allow(dead_code)]
pub struct WebSocketFuzzer;

impl WebSocketFuzzer {
    pub fn test_payloads() -> Vec<String> {
        vec![
            r#"{"type":"FUZZ"}"#.to_string(),
            r#"{"action":"FUZZ"}"#.to_string(),
            r#"{"cmd":"FUZZ"}"#.to_string(),
        ]
    }
    
    pub fn protocol_upgrade_headers() -> Vec<(String, String)> {
        vec![
            ("Upgrade", "websocket"),
            ("Connection", "Upgrade"),
            ("Sec-WebSocket-Version", "13"),
            ("Sec-WebSocket-Key", "dGhlIHNhbXBsZSBub25jZQ=="),
        ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }
}
