use std::time::{Duration, Instant};
use reqwest::Response;

/// Result of a single request execution
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub status_code: Option<u16>,
    pub response_time: Duration,
    pub body_size: usize,
    pub error: Option<String>,
    pub url: String,
    pub payload: String,
}

impl ExecutionResult {
    pub fn new_success(
        status_code: u16,
        response_time: Duration,
        body_size: usize,
        url: String,
        payload: String,
    ) -> Self {
        Self {
            success: true,
            status_code: Some(status_code),
            response_time,
            body_size,
            error: None,
            url,
            payload,
        }
    }
    
    pub fn new_error(url: String, payload: String, error: String) -> Self {
        Self {
            success: false,
            status_code: None,
            response_time: Duration::from_secs(0),
            body_size: 0,
            error: Some(error),
            url,
            payload,
        }
    }
}

/// Execute HTTP requests with timing and error handling
pub struct RequestExecutor {
    timeout: Duration,
    retry_count: usize,
    retry_delay: Duration,
}

impl RequestExecutor {
    pub fn new(timeout: Duration) -> Self {
        Self {
            timeout,
            retry_count: 0,
            retry_delay: Duration::from_millis(100),
        }
    }
    
    pub fn with_retry(mut self, count: usize, delay: Duration) -> Self {
        self.retry_count = count;
        self.retry_delay = delay;
        self
    }
    
    /// Execute a single request with timing
    pub async fn execute(
        &self,
        client: &reqwest::Client,
        url: &str,
        payload: &str,
    ) -> ExecutionResult {
        let start = Instant::now();
        
        match self.execute_with_retry(client, url).await {
            Ok(response) => {
                let duration = start.elapsed();
                let status = response.status().as_u16();
                let body = response.text().await.unwrap_or_default();
                let body_size = body.len();
                
                ExecutionResult::new_success(
                    status,
                    duration,
                    body_size,
                    url.to_string(),
                    payload.to_string(),
                )
            }
            Err(e) => ExecutionResult::new_error(
                url.to_string(),
                payload.to_string(),
                e.to_string(),
            ),
        }
    }
    
    async fn execute_with_retry(
        &self,
        client: &reqwest::Client,
        url: &str,
    ) -> Result<Response, reqwest::Error> {
        let mut attempts = 0;
        
        loop {
            match client.get(url).timeout(self.timeout).send().await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    attempts += 1;
                    if attempts > self.retry_count {
                        return Err(e);
                    }
                    tokio::time::sleep(self.retry_delay).await;
                }
            }
        }
    }
    
    /// Execute multiple requests in batch
    pub async fn execute_batch(
        &self,
        client: &reqwest::Client,
        requests: Vec<(String, String)>, // (url, payload) pairs
    ) -> Vec<ExecutionResult> {
        let mut results = Vec::new();
        
        for (url, payload) in requests {
            let result = self.execute(client, &url, &payload).await;
            results.push(result);
        }
        
        results
    }
}

impl Default for RequestExecutor {
    fn default() -> Self {
        Self::new(Duration::from_secs(10))
    }
}
