#![allow(dead_code)]
pub struct HeaderFuzzer;

impl HeaderFuzzer {
    pub fn dangerous_headers() -> Vec<(&'static str, &'static str)> {
        vec![
            ("X-Forwarded-For", "127.0.0.1"),
            ("X-Forwarded-Host", "evil.com"),
            ("X-Original-URL", "/admin"),
            ("X-Rewrite-URL", "/admin"),
            ("X-Originating-IP", "127.0.0.1"),
            ("X-Remote-IP", "127.0.0.1"),
            ("X-Client-IP", "127.0.0.1"),
            ("X-Host", "evil.com"),
            ("X-Forwarded-Server", "evil.com"),
            ("Forwarded", "for=127.0.0.1;host=evil.com"),
        ]
    }
    
    pub fn injection_headers() -> Vec<&'static str> {
        vec![
            "User-Agent",
            "Referer",
            "Cookie",
            "X-Custom-Header",
            "Accept-Language",
            "Accept-Encoding",
        ]
    }
    
    pub fn method_override_headers() -> Vec<(&'static str, &'static str)> {
        vec![
            ("X-HTTP-Method-Override", "PUT"),
            ("X-HTTP-Method-Override", "DELETE"),
            ("X-HTTP-Method", "PUT"),
            ("X-Method-Override", "DELETE"),
            ("_method", "PUT"),
        ]
    }
}
