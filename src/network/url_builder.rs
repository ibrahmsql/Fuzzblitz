/// URL building and manipulation utilities

pub fn build_url(url_template: &str, replacements: &[(String, String)]) -> String {
    let mut url = url_template.to_string();
    
    for (keyword, value) in replacements {
        url = url.replace(keyword, value);
    }
    
    url
}

pub fn add_extension(path: &str, extension: &str) -> String {
    if extension.starts_with('.') {
        format!("{}{}", path, extension)
    } else {
        format!("{}.{}", path, extension)
    }
}

pub struct UrlBuilder {
    template: String,
}

impl UrlBuilder {
    pub fn new(template: String) -> Self {
        Self { template }
    }
    
    pub fn build(&self, replacements: &[(String, String)]) -> String {
        build_url(&self.template, replacements)
    }
    
    pub fn with_extension(&self, replacements: &[(String, String)], extension: &str) -> String {
        let base = self.build(replacements);
        add_extension(&base, extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let replacements = vec![
            ("FUZZ".to_string(), "admin".to_string()),
            ("PORT".to_string(), "8080".to_string()),
        ];
        let result = build_url("http://example.com:PORT/FUZZ", &replacements);
        assert_eq!(result, "http://example.com:8080/admin");
    }

    #[test]
    fn test_add_extension() {
        assert_eq!(add_extension("index", ".php"), "index.php");
        assert_eq!(add_extension("index", "php"), "index.php");
    }

    #[test]
    fn test_url_builder() {
        let builder = UrlBuilder::new("http://example.com/FUZZ".to_string());
        let replacements = vec![("FUZZ".to_string(), "test".to_string())];
        assert_eq!(builder.build(&replacements), "http://example.com/test");
    }
}
