#![allow(dead_code)]
use serde_json::{json, Value};

pub struct GraphQLMutations;

impl GraphQLMutations {
    pub fn simple_mutation(name: &str, args: &str) -> Value {
        json!({
            "query": format!("mutation {{ {}({}) }}", name, args)
        })
    }
    
    pub fn injection_payloads() -> Vec<String> {
        vec![
            "' OR '1'='1".to_string(),
            "\") OR 1=1--".to_string(),
            "'; DROP TABLE users--".to_string(),
        ]
    }
}
