pub struct CookieParser;

impl CookieParser {
    pub fn parse(cookie_string: &str) -> Vec<(String, String)> {
        cookie_string.split(';')
            .filter_map(|pair| {
                let parts: Vec<&str> = pair.trim().splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect()
    }
    
    pub fn serialize(cookies: &[(String, String)]) -> String {
        cookies.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("; ")
    }
    
    pub fn common_cookie_names() -> Vec<String> {
        vec![
            "session", "SESSION", "sessionid", "SESSIONID",
            "PHPSESSID", "JSESSIONID", "ASP.NET_SessionId",
            "auth", "AUTH", "token", "TOKEN",
            "csrf", "CSRF", "_csrf", "csrf_token",
        ].iter().map(|s| s.to_string()).collect()
    }
}
