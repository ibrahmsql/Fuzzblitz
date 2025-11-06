use std::sync::Arc;
use tokio::time::{sleep, Duration};
use super::executor::{RequestExecutor, ExecutionResult};

/// Run requests sequentially with optional delays
pub struct SequentialRunner {
    executor: Arc<RequestExecutor>,
    delay: Option<Duration>,
}

impl SequentialRunner {
    pub fn new(executor: RequestExecutor) -> Self {
        Self {
            executor: Arc::new(executor),
            delay: None,
        }
    }
    
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }
    
    /// Run requests sequentially
    pub async fn run(
        &self,
        client: Arc<reqwest::Client>,
        requests: Vec<(String, String)>,
    ) -> Vec<ExecutionResult> {
        let mut results = Vec::new();
        
        for (url, payload) in requests {
            let result = self.executor.execute(&client, &url, &payload).await;
            results.push(result);
            
            if let Some(delay) = self.delay {
                sleep(delay).await;
            }
        }
        
        results
    }
    
    /// Run with progress callback
    pub async fn run_with_progress<F>(
        &self,
        client: Arc<reqwest::Client>,
        requests: Vec<(String, String)>,
        mut progress_callback: F,
    ) -> Vec<ExecutionResult>
    where
        F: FnMut(usize, usize),
    {
        let total = requests.len();
        let mut results = Vec::new();
        
        for (idx, (url, payload)) in requests.into_iter().enumerate() {
            let result = self.executor.execute(&client, &url, &payload).await;
            results.push(result);
            progress_callback(idx + 1, total);
            
            if let Some(delay) = self.delay {
                sleep(delay).await;
            }
        }
        
        results
    }
}
