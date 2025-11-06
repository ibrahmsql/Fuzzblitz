use reqwest::{Client, Method, Proxy, header::{HeaderMap, HeaderName, HeaderValue}};
use std::time::Duration;
use std::str::FromStr;
use crate::cli::ProgramArgs;

pub struct FuzzClient {
    client: Client,
    method: Method,
    headers: HeaderMap,
    cookies: Option<String>,
    data: Option<String>,
}

impl FuzzClient {
    pub fn from_args(args: &ProgramArgs) -> Result<Self, Box<dyn std::error::Error>> {
        let mut client_builder = Client::builder()
            .timeout(Duration::from_secs(args.timeout))
            .danger_accept_invalid_certs(true); // For testing purposes
        
        // HTTP/2 support
        if args.http2 {
            client_builder = client_builder.http2_prior_knowledge();
        }
        
        // Redirect policy
        if args.follow_redirects {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::limited(10));
        } else {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
        }
        
        // Proxy support
        if let Some(ref proxy_url) = args.proxy {
            let proxy = if proxy_url.starts_with("socks5://") {
                Proxy::all(proxy_url)?
            } else {
                Proxy::all(proxy_url)?
            };
            client_builder = client_builder.proxy(proxy);
        }
        
        let client = client_builder.build()?;
        
        // Parse HTTP method
        let method = Method::from_str(&args.method.to_uppercase())
            .unwrap_or(Method::GET);
        
        // Parse headers
        let mut headers = HeaderMap::new();
        for header_str in &args.headers {
            if let Some(pos) = header_str.find(':') {
                let name = header_str[..pos].trim();
                let value = header_str[pos + 1..].trim();
                
                if let (Ok(header_name), Ok(header_value)) = (
                    HeaderName::from_str(name),
                    HeaderValue::from_str(value)
                ) {
                    headers.insert(header_name, header_value);
                }
            }
        }
        
        // Add default User-Agent if not specified
        if !headers.contains_key("user-agent") {
            headers.insert(
                HeaderName::from_static("user-agent"),
                HeaderValue::from_static("RustFuzz/2.0")
            );
        }
        
        Ok(Self {
            client,
            method,
            headers,
            cookies: args.cookies.clone(),
            data: args.data.clone(),
        })
    }
    
    pub async fn send_request(
        &self,
        url: &str,
        ignore_body: bool,
    ) -> Result<(u16, String), Box<dyn std::error::Error>> {
        let mut request = self.client
            .request(self.method.clone(), url)
            .headers(self.headers.clone());
        
        // Add cookies
        if let Some(ref cookies) = self.cookies {
            request = request.header("Cookie", cookies);
        }
        
        // Add POST data
        if let Some(ref data) = self.data {
            request = request.body(data.clone());
        }
        
        let response = request.send().await?;
        let status_code = response.status().as_u16();
        
        let body = if ignore_body {
            String::new()
        } else {
            response.text().await?
        };
        
        Ok((status_code, body))
    }
}

