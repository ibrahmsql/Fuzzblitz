#![allow(dead_code)]
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use super::task::Job;

/// Thread-safe job queue
pub struct JobQueue {
    queue: Arc<Mutex<VecDeque<Job>>>,
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }
    
    /// Add a job to the queue
    pub fn push(&self, job: Job) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(job);
    }
    
    /// Get the next job from the queue
    pub fn pop(&self) -> Option<Job> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
    
    /// Peek at the next job without removing it
    pub fn peek(&self) -> Option<Job> {
        let queue = self.queue.lock().unwrap();
        queue.front().cloned()
    }
    
    /// Get the current queue size
    pub fn len(&self) -> usize {
        let queue = self.queue.lock().unwrap();
        queue.len()
    }
    
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        let queue = self.queue.lock().unwrap();
        queue.is_empty()
    }
    
    /// Clear all jobs from the queue
    pub fn clear(&self) {
        let mut queue = self.queue.lock().unwrap();
        queue.clear();
    }
    
    /// Get all jobs as a vector
    pub fn all_jobs(&self) -> Vec<Job> {
        let queue = self.queue.lock().unwrap();
        queue.iter().cloned().collect()
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for JobQueue {
    fn clone(&self) -> Self {
        Self {
            queue: Arc::clone(&self.queue),
        }
    }
}
