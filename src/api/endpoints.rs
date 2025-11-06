#![allow(dead_code)]
pub struct EndpointDiscovery;

impl EndpointDiscovery {
    pub fn common_api_endpoints() -> Vec<String> {
        vec![
            "/users", "/user", "/accounts", "/account",
            "/login", "/signin", "/auth", "/authenticate",
            "/register", "/signup", "/create-account",
            "/profile", "/me", "/settings",
            "/admin", "/dashboard", "/panel",
            "/api", "/rest", "/v1", "/v2",
            "/graphql", "/query",
            "/upload", "/download", "/file",
            "/search", "/find", "/query",
            "/delete", "/remove", "/update",
            "/config", "/configuration", "/settings",
        ].iter().map(|s| s.to_string()).collect()
    }
    
    pub fn generate_crud_endpoints(resource: &str) -> Vec<String> {
        vec![
            format!("/{}", resource),
            format!("/{}/{{id}}", resource),
            format!("/{}/create", resource),
            format!("/{}/update", resource),
            format!("/{}/delete", resource),
            format!("/{}/list", resource),
        ]
    }
}
