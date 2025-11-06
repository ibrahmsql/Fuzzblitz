pub struct GraphQLIntrospection;

impl GraphQLIntrospection {
    pub fn is_enabled(_response: &str) -> bool {
        true
    }
    
    pub fn extract_types(_response: &str) -> Vec<String> {
        Vec::new()
    }
}
