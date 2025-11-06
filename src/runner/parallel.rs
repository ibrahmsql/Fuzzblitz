#![allow(dead_code)]
use futures::stream::{self, StreamExt};
use std::sync::Arc;
use super::executor::{RequestExecutor, ExecutionResult};

/// Run requests in parallel with configurable concurrency
pub struct ParallelRunner {
    executor: Arc<RequestExecutor>,
    concurrency: usize,
}

impl ParallelRunner {
    pub fn new(executor: RequestExecutor, concurrency: usize) -> Self {
        Self {
            executor: Arc::new(executor),
            concurrency: concurrency.max(1),
        }
    }
    
    /// Run requests in parallel
    pub async fn run(
        &self,
        client: Arc<reqwest::Client>,
        requests: Vec<(String, String)>,
    ) -> Vec<ExecutionResult> {
        stream::iter(requests)
            .map(|(url, payload)| {
                let executor = Arc::clone(&self.executor);
                let client = Arc::clone(&client);
                async move {
                    executor.execute(&client, &url, &payload).await
                }
            })
            .buffer_unordered(self.concurrency)
            .collect()
            .await
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
        let mut completed = 0;
        
        let results = stream::iter(requests)
            .map(|(url, payload)| {
                let executor = Arc::clone(&self.executor);
                let client = Arc::clone(&client);
                async move {
                    executor.execute(&client, &url, &payload).await
                }
            })
            .buffer_unordered(self.concurrency)
            .map(|result| {
                completed += 1;
                progress_callback(completed, total);
                result
            })
            .collect()
            .await;
        
        results
    }
}
