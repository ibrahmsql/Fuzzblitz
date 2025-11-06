pub struct WafSignatures;

impl WafSignatures {
    pub fn test_payloads() -> Vec<String> {
        vec![
            "<script>alert(1)</script>".to_string(),
            "' OR '1'='1".to_string(),
            "../../../etc/passwd".to_string(),
            "{{7*7}}".to_string(),
            "${7*7}".to_string(),
        ]
    }
    
    pub fn get_waf_list() -> Vec<&'static str> {
        vec![
            "Cloudflare",
            "AWS WAF",
            "Akamai",
            "ModSecurity",
            "Imperva",
            "F5 BIG-IP",
            "Sucuri",
            "Wordfence",
            "Barracuda",
            "Fortinet FortiWeb",
        ]
    }
}
