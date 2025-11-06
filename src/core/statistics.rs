use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone)]
pub struct Statistics {
    total: usize,
    completed: Arc<AtomicUsize>,
    matched: Arc<AtomicUsize>,
    errors: Arc<AtomicUsize>,
    start_time: Instant,
    bytes_sent: Arc<AtomicU64>,
    bytes_received: Arc<AtomicU64>,
}

impl Statistics {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            completed: Arc::new(AtomicUsize::new(0)),
            matched: Arc::new(AtomicUsize::new(0)),
            errors: Arc::new(AtomicUsize::new(0)),
            start_time: Instant::now(),
            bytes_sent: Arc::new(AtomicU64::new(0)),
            bytes_received: Arc::new(AtomicU64::new(0)),
        }
    }
    
    pub fn increment_completed(&self) {
        self.completed.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn increment_matched(&self) {
        self.matched.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn increment_errors(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn add_bytes_sent(&self, bytes: u64) {
        self.bytes_sent.fetch_add(bytes, Ordering::Relaxed);
    }
    
    pub fn add_bytes_received(&self, bytes: u64) {
        self.bytes_received.fetch_add(bytes, Ordering::Relaxed);
    }
    
    pub fn total(&self) -> usize {
        self.total
    }
    
    pub fn completed(&self) -> usize {
        self.completed.load(Ordering::Relaxed)
    }
    
    pub fn matched(&self) -> usize {
        self.matched.load(Ordering::Relaxed)
    }
    
    pub fn errors(&self) -> usize {
        self.errors.load(Ordering::Relaxed)
    }
    
    pub fn elapsed_secs(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
    
    pub fn elapsed_millis(&self) -> u128 {
        self.start_time.elapsed().as_millis()
    }
    
    pub fn req_per_sec(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.completed() as f64 / elapsed
        } else {
            0.0
        }
    }
    
    pub fn bytes_sent(&self) -> u64 {
        self.bytes_sent.load(Ordering::Relaxed)
    }
    
    pub fn bytes_received(&self) -> u64 {
        self.bytes_received.load(Ordering::Relaxed)
    }
    
    pub fn bytes_sent_mb(&self) -> f64 {
        self.bytes_sent() as f64 / 1_048_576.0
    }
    
    pub fn bytes_received_mb(&self) -> f64 {
        self.bytes_received() as f64 / 1_048_576.0
    }
    
    pub fn progress_percent(&self) -> f64 {
        if self.total > 0 {
            (self.completed() as f64 / self.total as f64) * 100.0
        } else {
            0.0
        }
    }
    
    pub fn match_rate(&self) -> f64 {
        let completed = self.completed();
        if completed > 0 {
            (self.matched() as f64 / completed as f64) * 100.0
        } else {
            0.0
        }
    }
    
    pub fn error_rate(&self) -> f64 {
        let completed = self.completed();
        if completed > 0 {
            (self.errors() as f64 / completed as f64) * 100.0
        } else {
            0.0
        }
    }
}
