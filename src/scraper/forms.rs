use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FormData {
    pub action: String,
    pub method: String,
    pub inputs: Vec<InputField>,
}

#[derive(Debug, Clone)]
pub struct InputField {
    pub name: String,
    pub input_type: String,
    pub value: Option<String>,
}

/// Extract forms from HTML
pub struct FormExtractor;

impl FormExtractor {
    /// Extract all forms from HTML
    pub fn extract_forms(html: &str) -> Vec<FormData> {
        let form_re = Regex::new(r"<form[^>]*>(.*?)</form>").unwrap();
        let action_re = Regex::new(r#"action="([^"]+)""#).unwrap();
        let method_re = Regex::new(r#"method="([^"]+)""#).unwrap();
        
        form_re.captures_iter(html)
            .filter_map(|cap| {
                let form_html = cap.get(0)?.as_str();
                let form_content = cap.get(1)?.as_str();
                
                let action = action_re.captures(form_html)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();
                
                let method = method_re.captures(form_html)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_uppercase())
                    .unwrap_or_else(|| "GET".to_string());
                
                let inputs = Self::extract_inputs(form_content);
                
                Some(FormData {
                    action,
                    method,
                    inputs,
                })
            })
            .collect()
    }
    
    /// Extract input fields from form HTML
    pub fn extract_inputs(html: &str) -> Vec<InputField> {
        let input_re = Regex::new(r#"<input[^>]+>"#).unwrap();
        let name_re = Regex::new(r#"name="([^"]+)""#).unwrap();
        let type_re = Regex::new(r#"type="([^"]+)""#).unwrap();
        let value_re = Regex::new(r#"value="([^"]+)""#).unwrap();
        
        input_re.find_iter(html)
            .filter_map(|m| {
                let input_html = m.as_str();
                
                let name = name_re.captures(input_html)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string())?;
                
                let input_type = type_re.captures(input_html)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_else(|| "text".to_string());
                
                let value = value_re.captures(input_html)
                    .and_then(|c| c.get(1))
                    .map(|m| m.as_str().to_string());
                
                Some(InputField {
                    name,
                    input_type,
                    value,
                })
            })
            .collect()
    }
    
    /// Generate fuzzing data from forms
    pub fn generate_fuzz_data(forms: &[FormData]) -> Vec<HashMap<String, String>> {
        forms.iter()
            .map(|form| {
                let mut data = HashMap::new();
                for input in &form.inputs {
                    data.insert(input.name.clone(), input.value.clone().unwrap_or_else(|| "FUZZ".to_string()));
                }
                data
            })
            .collect()
    }
}
