#![allow(dead_code)]
use std::sync::{Arc, Mutex};

pub struct StopConditions {
    stop_on_all_errors: bool,
    stop_on_spurious_errors: bool,
    stop_on_403_threshold: bool,
    
    total_requests: Arc<Mutex<usize>>,
    error_count: Arc<Mutex<usize>>,
    forbidden_count: Arc<Mutex<usize>>,
    should_stop: Arc<Mutex<bool>>,
}

impl StopConditions {
    pub fn new(stop_all: bool, stop_spurious: bool, stop_403: bool) -> Self {
        Self {
            stop_on_all_errors: stop_all,
            stop_on_spurious_errors: stop_spurious,
            stop_on_403_threshold: stop_403,
            total_requests: Arc::new(Mutex::new(0)),
            error_count: Arc::new(Mutex::new(0)),
            forbidden_count: Arc::new(Mutex::new(0)),
            should_stop: Arc::new(Mutex::new(false)),
        }
    }
    
    pub fn check_response(&self, status_code: Option<u16>, is_error: bool) {
        let mut total = self.total_requests.lock().unwrap();
        *total += 1;
        
        if is_error {
            let mut errors = self.error_count.lock().unwrap();
            *errors += 1;
            
            if self.stop_on_all_errors || self.stop_on_spurious_errors {
                let mut should_stop = self.should_stop.lock().unwrap();
                *should_stop = true;
            }
        }
        
        if let Some(403) = status_code {
            let mut forbidden = self.forbidden_count.lock().unwrap();
            *forbidden += 1;
            
            if self.stop_on_403_threshold && *total > 10 {
                let forbidden_rate = *forbidden as f64 / *total as f64;
                if forbidden_rate > 0.95 {
                    let mut should_stop = self.should_stop.lock().unwrap();
                    *should_stop = true;
                }
            }
        }
    }
    
    pub fn should_stop(&self) -> bool {
        *self.should_stop.lock().unwrap()
    }
    
    pub fn get_stats(&self) -> (usize, usize, usize) {
        let total = *self.total_requests.lock().unwrap();
        let errors = *self.error_count.lock().unwrap();
        let forbidden = *self.forbidden_count.lock().unwrap();
        (total, errors, forbidden)
    }
}
