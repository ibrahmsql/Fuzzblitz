#![allow(dead_code)]
use serde_json::{json, Value};

pub struct GraphQLQueries;

impl GraphQLQueries {
    pub fn introspection_query() -> Value {
        json!({
            "query": "query IntrospectionQuery { __schema { queryType { name } mutationType { name } types { ...FullType } } } fragment FullType on __Type { kind name description fields(includeDeprecated: true) { name description args { ...InputValue } type { ...TypeRef } isDeprecated deprecationReason } }"
        })
    }
    
    pub fn simple_query(field: &str) -> Value {
        json!({
            "query": format!("{{ {} }}", field)
        })
    }
    
    pub fn query_with_args(field: &str, args: &str) -> Value {
        json!({
            "query": format!("{{ {}({}) }}", field, args)
        })
    }
    
    pub fn batched_queries(queries: Vec<String>) -> Value {
        let q: Vec<Value> = queries.iter()
            .map(|q| json!({"query": q}))
            .collect();
        json!(q)
    }
}
