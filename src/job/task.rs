use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub id: usize,
    pub url: String,
    pub wordlist: String,
    pub status: JobStatus,
    pub created_at: Instant,
    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,
    pub total_requests: usize,
    pub completed_requests: usize,
    pub matched_results: usize,
}

impl Job {
    pub fn new(id: usize, url: String, wordlist: String, total_requests: usize) -> Self {
        Self {
            id,
            url,
            wordlist,
            status: JobStatus::Pending,
            created_at: Instant::now(),
            started_at: None,
            finished_at: None,
            total_requests,
            completed_requests: 0,
            matched_results: 0,
        }
    }
    
    pub fn start(&mut self) {
        self.status = JobStatus::Running;
        self.started_at = Some(Instant::now());
    }
    
    pub fn complete(&mut self) {
        self.status = JobStatus::Completed;
        self.finished_at = Some(Instant::now());
    }
    
    pub fn fail(&mut self) {
        self.status = JobStatus::Failed;
        self.finished_at = Some(Instant::now());
    }
    
    pub fn cancel(&mut self) {
        self.status = JobStatus::Cancelled;
        self.finished_at = Some(Instant::now());
    }
    
    pub fn progress(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        (self.completed_requests as f64 / self.total_requests as f64) * 100.0
    }
    
    pub fn elapsed(&self) -> Duration {
        if let Some(started) = self.started_at {
            if let Some(finished) = self.finished_at {
                finished.duration_since(started)
            } else {
                Instant::now().duration_since(started)
            }
        } else {
            Duration::from_secs(0)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobResult {
    pub job_id: usize,
    pub total_requests: usize,
    pub matched_results: usize,
    pub elapsed_secs: u64,
    pub req_per_sec: f64,
}

impl JobResult {
    pub fn from_job(job: &Job) -> Self {
        let elapsed = job.elapsed().as_secs();
        let req_per_sec = if elapsed > 0 {
            job.completed_requests as f64 / elapsed as f64
        } else {
            0.0
        };
        
        Self {
            job_id: job.id,
            total_requests: job.total_requests,
            matched_results: job.matched_results,
            elapsed_secs: elapsed,
            req_per_sec,
        }
    }
}
