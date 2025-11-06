use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::task::{Job, JobStatus};
use super::queue::JobQueue;

/// Manage multiple fuzzing jobs
pub struct JobManager {
    jobs: Arc<Mutex<HashMap<usize, Job>>>,
    queue: JobQueue,
    next_id: Arc<Mutex<usize>>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            queue: JobQueue::new(),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
    
    /// Create a new job
    pub fn create_job(&self, url: String, wordlist: String, total_requests: usize) -> usize {
        let mut id_counter = self.next_id.lock().unwrap();
        let job_id = *id_counter;
        *id_counter += 1;
        
        let job = Job::new(job_id, url, wordlist, total_requests);
        
        let mut jobs = self.jobs.lock().unwrap();
        jobs.insert(job_id, job.clone());
        
        self.queue.push(job);
        
        job_id
    }
    
    /// Get a job by ID
    pub fn get_job(&self, job_id: usize) -> Option<Job> {
        let jobs = self.jobs.lock().unwrap();
        jobs.get(&job_id).cloned()
    }
    
    /// Update a job
    pub fn update_job(&self, job_id: usize, updater: impl FnOnce(&mut Job)) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(job) = jobs.get_mut(&job_id) {
            updater(job);
        }
    }
    
    /// Start a job
    pub fn start_job(&self, job_id: usize) {
        self.update_job(job_id, |job| job.start());
    }
    
    /// Complete a job
    pub fn complete_job(&self, job_id: usize) {
        self.update_job(job_id, |job| job.complete());
    }
    
    /// Fail a job
    pub fn fail_job(&self, job_id: usize) {
        self.update_job(job_id, |job| job.fail());
    }
    
    /// Cancel a job
    pub fn cancel_job(&self, job_id: usize) {
        self.update_job(job_id, |job| job.cancel());
    }
    
    /// Get all jobs
    pub fn all_jobs(&self) -> Vec<Job> {
        let jobs = self.jobs.lock().unwrap();
        jobs.values().cloned().collect()
    }
    
    /// Get jobs by status
    pub fn jobs_by_status(&self, status: JobStatus) -> Vec<Job> {
        let jobs = self.jobs.lock().unwrap();
        jobs.values()
            .filter(|job| matches!(job.status, ref s if std::mem::discriminant(s) == std::mem::discriminant(&status)))
            .cloned()
            .collect()
    }
    
    /// Get the next job from the queue
    pub fn next_job(&self) -> Option<Job> {
        self.queue.pop()
    }
    
    /// Get total job count
    pub fn job_count(&self) -> usize {
        let jobs = self.jobs.lock().unwrap();
        jobs.len()
    }
    
    /// Clear completed jobs
    pub fn clear_completed(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        jobs.retain(|_, job| !matches!(job.status, JobStatus::Completed));
    }
}

impl Default for JobManager {
    fn default() -> Self {
        Self::new()
    }
}
