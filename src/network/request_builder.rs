#![allow(dead_code)]
use reqwest::{Client, Request, Method, header::{HeaderMap, HeaderName, HeaderValue}};
use std::str::FromStr;
use std::collections::HashMap;

pub struct RequestBuilder {
    client: Client,
    method: Method,
    headers: HeaderMap,
    cookies: Option<String>,
    body: Option<Vec<u8>>,
}

impl RequestBuilder {
    pub fn new(
        client: Client,
        method: &str,
        headers: Vec<String>,
        cookies: Option<String>,
        data: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let method = Method::from_str(&method.to_uppercase()).unwrap_or(Method::GET);
        
        let mut header_map = HeaderMap::new();
        for header_str in headers {
            if let Some(pos) = header_str.find(':') {
                let name = header_str[..pos].trim();
                let value = header_str[pos + 1..].trim();
                
                if let (Ok(header_name), Ok(header_value)) = (
                    HeaderName::from_str(name),
                    HeaderValue::from_str(value)
                ) {
                    header_map.insert(header_name, header_value);
                }
            }
        }
        
        if !header_map.contains_key("user-agent") {
            header_map.insert(
                HeaderName::from_static("user-agent"),
                HeaderValue::from_static("RustFuzz/2.0")
            );
        }
        
        let body = data.map(|d| d.into_bytes());
        
        Ok(Self {
            client,
            method,
            headers: header_map,
            cookies,
            body,
        })
    }
    
    pub fn build_request(&self, url: &str, custom_body: Option<Vec<u8>>) -> Result<Request, Box<dyn std::error::Error>> {
        let mut request = self.client
            .request(self.method.clone(), url)
            .headers(self.headers.clone());
        
        if let Some(ref cookies) = self.cookies {
            request = request.header("Cookie", cookies);
        }
        
        if let Some(body_data) = custom_body.or_else(|| self.body.clone()) {
            request = request.body(body_data);
        }
        
        Ok(request.build()?)
    }
    
    pub fn build_request_with_replacements(
        &self,
        url: &str,
        replacements: &HashMap<String, String>,
    ) -> Result<Request, Box<dyn std::error::Error>> {
        let mut final_url = url.to_string();
        let mut final_headers = self.headers.clone();
        let mut final_cookies = self.cookies.clone();
        let mut final_body = self.body.clone();
        
        for (keyword, value) in replacements {
            final_url = final_url.replace(keyword, value);
            
            if let Some(ref cookies) = final_cookies {
                final_cookies = Some(cookies.replace(keyword, value));
            }
            
            if let Some(ref body) = final_body {
                let body_str = String::from_utf8_lossy(body);
                let replaced = body_str.replace(keyword, value);
                final_body = Some(replaced.into_bytes());
            }
            
            let mut new_headers = HeaderMap::new();
            for (name, val) in final_headers.iter() {
                let val_str = val.to_str().unwrap_or("");
                let replaced = val_str.replace(keyword, value);
                if let Ok(new_val) = HeaderValue::from_str(&replaced) {
                    new_headers.insert(name.clone(), new_val);
                }
            }
            final_headers = new_headers;
        }
        
        let mut request = self.client
            .request(self.method.clone(), &final_url)
            .headers(final_headers);
        
        if let Some(ref cookies) = final_cookies {
            request = request.header("Cookie", cookies);
        }
        
        if let Some(body_data) = final_body {
            request = request.body(body_data);
        }
        
        Ok(request.build()?)
    }
}
