#![allow(dead_code)]
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use crate::core::Statistics;

/// Interactive console for runtime control
pub struct InteractiveConsole {
    enabled: bool,
    stats: Arc<Statistics>,
    paused: Arc<Mutex<bool>>,
}

impl InteractiveConsole {
    pub fn new(enabled: bool, stats: Arc<Statistics>) -> Self {
        Self {
            enabled,
            stats,
            paused: Arc::new(Mutex::new(false)),
        }
    }
    
    /// Start the interactive console
    pub fn start(&self) {
        if !self.enabled {
            return;
        }
        
        println!("\n{}", "═".repeat(60));
        println!("Interactive Mode: Press 'h' for help");
        println!("{}", "═".repeat(60));
    }
    
    /// Process user input
    pub fn process_input(&self, input: &str) -> bool {
        match input.trim().to_lowercase().as_str() {
            "h" | "help" => {
                self.print_help();
                true
            }
            "s" | "stats" => {
                self.print_stats();
                true
            }
            "p" | "pause" => {
                self.toggle_pause();
                true
            }
            "r" | "resume" => {
                self.toggle_pause();
                true
            }
            "q" | "quit" => {
                println!("Quitting...");
                false
            }
            _ => {
                println!("Unknown command. Press 'h' for help.");
                true
            }
        }
    }
    
    fn print_help(&self) {
        println!("\n{}", "═".repeat(60));
        println!("Interactive Commands:");
        println!("  h, help   - Show this help");
        println!("  s, stats  - Show current statistics");
        println!("  p, pause  - Pause/Resume scanning");
        println!("  r, resume - Resume scanning");
        println!("  q, quit   - Quit the scan");
        println!("{}", "═".repeat(60));
    }
    
    fn print_stats(&self) {
        println!("\n{}", "═".repeat(60));
        println!("Current Statistics:");
        println!("  Total:    {}", self.stats.total());
        println!("  Matched:  {}", self.stats.matched());
        println!("  Rate:     {:.2} req/sec", self.stats.req_per_sec());
        println!("  Elapsed:  {}s", self.stats.elapsed_secs());
        println!("{}", "═".repeat(60));
    }
    
    fn toggle_pause(&self) {
        let mut paused = self.paused.lock().unwrap();
        *paused = !*paused;
        if *paused {
            println!("⏸  Scan PAUSED");
        } else {
            println!("▶  Scan RESUMED");
        }
    }
    
    pub fn is_paused(&self) -> bool {
        *self.paused.lock().unwrap()
    }
}

/// Read a single line from stdin
pub fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input)
}
