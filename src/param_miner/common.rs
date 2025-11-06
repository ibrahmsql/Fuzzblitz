#![allow(dead_code)]
pub struct CommonParams;

impl CommonParams {
    pub fn get_common_params() -> Vec<String> {
        vec![
            "id", "user", "username", "email", "password", "token",
            "page", "limit", "offset", "sort", "order", "filter",
            "search", "q", "query", "keyword", "term",
            "callback", "redirect", "return", "url", "next",
            "debug", "test", "admin", "dev", "mode",
            "api_key", "access_token", "auth", "session",
            "file", "path", "dir", "folder", "document",
            "lang", "language", "locale", "country", "region",
        ].iter().map(|s| s.to_string()).collect()
    }
    
    pub fn get_hidden_params() -> Vec<String> {
        vec![
            "debug", "_debug", "test", "_test",
            "admin", "_admin", "dev", "_dev",
            "internal", "_internal", "secret", "_secret",
            "hidden", "_hidden", "private", "_private",
        ].iter().map(|s| s.to_string()).collect()
    }
}
