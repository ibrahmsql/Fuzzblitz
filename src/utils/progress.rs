#![allow(dead_code)]
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use colored::Colorize;

pub struct LiveStats {
    start_time: Instant,
    total: usize,
    completed: Arc<Mutex<usize>>,
    matched: Arc<Mutex<usize>>,
    errors: Arc<Mutex<usize>>,
}

impl LiveStats {
    pub fn new(total: usize) -> Self {
        Self {
            start_time: Instant::now(),
            total,
            completed: Arc::new(Mutex::new(0)),
            matched: Arc::new(Mutex::new(0)),
            errors: Arc::new(Mutex::new(0)),
        }
    }
    
    pub fn increment_completed(&self) {
        let mut completed = self.completed.lock().unwrap();
        *completed += 1;
    }
    
    pub fn increment_matched(&self) {
        let mut matched = self.matched.lock().unwrap();
        *matched += 1;
    }
    
    pub fn increment_errors(&self) {
        let mut errors = self.errors.lock().unwrap();
        *errors += 1;
    }
    
    pub fn print_live(&self) {
        let completed = *self.completed.lock().unwrap();
        let matched = *self.matched.lock().unwrap();
        let errors = *self.errors.lock().unwrap();
        let elapsed = self.start_time.elapsed().as_secs();
        let req_per_sec = if elapsed > 0 { completed as f64 / elapsed as f64 } else { 0.0 };
        
        let progress_pct = (completed as f64 / self.total as f64 * 100.0) as usize;
        let bar_width = 40;
        let filled = (bar_width as f64 * (completed as f64 / self.total as f64)) as usize;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(bar_width - filled));
        
        print!("\r");
        print!("{} ", ":: Progress :".bright_blue().bold());
        print!("[{}{}] ", 
            format!("{}/{}", completed, self.total).bright_white(),
            format!(" {}%", progress_pct).bright_cyan()
        );
        print!("{} ", bar.bright_green());
        print!(":: {} req/sec ", format!("{:.0}", req_per_sec).bright_yellow());
        print!(":: {} matched ", matched.to_string().bright_green());
        print!(":: {} errors ", errors.to_string().bright_red());
        print!(":: {:02}:{:02}:{:02}   ", elapsed / 3600, (elapsed % 3600) / 60, elapsed % 60);
        
        use std::io::{self, Write};
        io::stdout().flush().unwrap();
    }
    
    pub fn print_final(&self) {
        println!();
        let completed = *self.completed.lock().unwrap();
        let matched = *self.matched.lock().unwrap();
        let errors = *self.errors.lock().unwrap();
        let elapsed = self.start_time.elapsed().as_secs();
        let req_per_sec = if elapsed > 0 { completed as f64 / elapsed as f64 } else { 0.0 };
        
        println!("{}", "________________________________________________".bright_cyan());
        println!();
        println!(" {} {}", "::".bright_blue(), "Scan completed successfully!".bright_green().bold());
        println!(" {} Total: {} | Matched: {} | Errors: {} | Rate: {:.0} req/s | Time: {}s",
            "::".bright_blue(),
            completed.to_string().bright_white(),
            matched.to_string().bright_green(),
            errors.to_string().bright_red(),
            req_per_sec,
            elapsed
        );
        println!("{}", "________________________________________________".bright_cyan());
    }
}
