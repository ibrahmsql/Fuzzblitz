use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};
use futures::stream::{self, StreamExt};
use rand::Rng;

use crate::cli::ProgramArgs;
use crate::network::{FuzzClient, build_url, add_extension};
use crate::filters::{MatcherFilter, FuzzResponse};
use crate::output::FuzzResult;
use crate::input::{encode_payload, PayloadGenerator};
use super::rate_limiter::RateLimiter;
use super::statistics::Statistics;

pub struct FuzzEngine {
    args: ProgramArgs,
    client: Arc<FuzzClient>,
    matcher: Arc<MatcherFilter>,
    rate_limiter: Arc<RateLimiter>,
    statistics: Arc<Statistics>,
    encoder_map: Arc<HashMap<String, Vec<String>>>,
    extensions: Vec<String>,
    delay_range: Option<(u64, u64)>,
}

impl FuzzEngine {
    pub fn new(
        args: ProgramArgs,
        client: Arc<FuzzClient>,
        matcher: Arc<MatcherFilter>,
        rate_limiter: Arc<RateLimiter>,
        statistics: Arc<Statistics>,
        encoder_map: Arc<HashMap<String, Vec<String>>>,
        extensions: Vec<String>,
        delay_range: Option<(u64, u64)>,
    ) -> Self {
        Self {
            args,
            client,
            matcher,
            rate_limiter,
            statistics,
            encoder_map,
            extensions,
            delay_range,
        }
    }

    pub async fn run(
        &self,
        payload_gen: PayloadGenerator,
    ) -> Vec<FuzzResult> {
        let url = self.args.url.as_ref().unwrap().clone();
        
        // Collect all payloads
        let mut all_payloads = Vec::new();
        for payload_map in payload_gen {
            all_payloads.push(payload_map);
        }
        
        // Process requests concurrently
        let results = stream::iter(all_payloads)
            .map(|payload_map| {
                let url = url.clone();
                let client = Arc::clone(&self.client);
                let matcher = Arc::clone(&self.matcher);
                let rate_limiter = self.rate_limiter.clone_limiter();
                let encoder_map = Arc::clone(&self.encoder_map);
                let args = self.args.clone();
                let extensions = self.extensions.clone();
                let delay_range = self.delay_range;
                let stats = Arc::clone(&self.statistics);
                
                async move {
                    let _guard = rate_limiter.acquire().await;
                    
                    // Apply delay if specified
                    if let Some((min_ms, max_ms)) = delay_range {
                        let delay_ms = if min_ms == max_ms {
                            min_ms
                        } else {
                            rand::thread_rng().gen_range(min_ms..=max_ms)
                        };
                        sleep(Duration::from_millis(delay_ms)).await;
                    }
                    
                    // Build replacements with encoding
                    let mut replacements = Vec::new();
                    for (keyword, value) in &payload_map {
                        let encoded_value = if let Some(encoders) = encoder_map.get(keyword) {
                            encode_payload(value, encoders)
                        } else {
                            value.clone()
                        };
                        replacements.push((keyword.clone(), encoded_value));
                    }
                    
                    // Try extensions if specified
                    let urls_to_try = if extensions.is_empty() {
                        vec![build_url(&url, &replacements)]
                    } else {
                        let base_url = build_url(&url, &replacements);
                        let mut urls = vec![base_url.clone()];
                        for ext in &extensions {
                            urls.push(add_extension(&base_url, ext));
                        }
                        urls
                    };
                    
                    let mut results = Vec::new();
                    for test_url in urls_to_try {
                        let start = Instant::now();
                        
                        match client.send_request(&test_url, args.ignore_body).await {
                            Ok((status_code, body)) => {
                                let response_time_ms = start.elapsed().as_millis() as i64;
                                
                                let fuzz_response = FuzzResponse::new(
                                    status_code,
                                    body,
                                    response_time_ms
                                );
                                
                                if matcher.should_show(&fuzz_response) {
                                    let keyword_str = payload_map.iter()
                                        .map(|(k, v)| format!("{}={}", k, v))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    
                                    results.push(FuzzResult::new(
                                        keyword_str,
                                        test_url,
                                        fuzz_response.status_code,
                                        fuzz_response.body_length,
                                        fuzz_response.lines,
                                        fuzz_response.words,
                                        response_time_ms,
                                    ));
                                }
                            },
                            Err(_) => {
                                // Silently skip errors or log if verbose
                            }
                        }
                    }
                    
                    stats.increment_completed();
                    results
                }
            })
            .buffer_unordered(self.args.threads)
            .collect::<Vec<_>>()
            .await;
        
        // Flatten results
        results.into_iter().flatten().collect()
    }
}
