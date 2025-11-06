#![allow(dead_code)]
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};

pub struct RateLimiter {
    semaphore: Arc<Semaphore>,
    rate_per_second: Option<u64>,
    last_request: Arc<AtomicU64>,
}

impl RateLimiter {
    pub fn new(threads: usize, rate_per_second: Option<u64>) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(threads)),
            rate_per_second,
            last_request: Arc::new(AtomicU64::new(0)),
        }
    }
    
    pub async fn acquire(&self) -> RateLimiterGuard {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        
        if let Some(rate) = self.rate_per_second {
            let min_interval_micros = 1_000_000 / rate;
            let now = Instant::now().elapsed().as_micros() as u64;
            let last = self.last_request.load(Ordering::Relaxed);
            
            if last > 0 {
                let elapsed = now.saturating_sub(last);
                if elapsed < min_interval_micros {
                    let wait_time = min_interval_micros - elapsed;
                    sleep(Duration::from_micros(wait_time)).await;
                }
            }
            
            self.last_request.store(Instant::now().elapsed().as_micros() as u64, Ordering::Relaxed);
        }
        
        RateLimiterGuard { _permit: permit }
    }
    
    pub fn clone_limiter(&self) -> Self {
        Self {
            semaphore: Arc::clone(&self.semaphore),
            rate_per_second: self.rate_per_second,
            last_request: Arc::clone(&self.last_request),
        }
    }
}

pub struct RateLimiterGuard {
    _permit: tokio::sync::OwnedSemaphorePermit,
}
