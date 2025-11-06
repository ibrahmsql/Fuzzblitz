pub struct SwaggerParser;

impl SwaggerParser {
    pub fn common_swagger_paths() -> Vec<String> {
        vec![
            "/swagger.json",
            "/swagger.yaml",
            "/swagger.yml",
            "/api/swagger.json",
            "/api/swagger.yaml",
            "/api-docs",
            "/api-docs/swagger.json",
            "/v2/api-docs",
            "/v3/api-docs",
            "/swagger-ui.html",
            "/swagger-ui/",
            "/api/swagger-ui.html",
            "/docs",
            "/openapi.json",
            "/openapi.yaml",
        ].iter().map(|s| s.to_string()).collect()
    }
    
    pub fn parse_endpoints(_swagger_content: &str) -> Vec<String> {
        Vec::new()
    }
}
