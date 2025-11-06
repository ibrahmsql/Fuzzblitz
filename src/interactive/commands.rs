#![allow(dead_code)]
/// Command types for interactive mode
#[derive(Debug, Clone)]
pub enum Command {
    Help,
    Stats,
    Pause,
    Resume,
    Quit,
    AddFilter(String),
    RemoveFilter(String),
    ChangeThreads(usize),
    Unknown(String),
}

impl Command {
    pub fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Command::Unknown(String::new());
        }
        
        match parts[0].to_lowercase().as_str() {
            "h" | "help" => Command::Help,
            "s" | "stats" => Command::Stats,
            "p" | "pause" => Command::Pause,
            "r" | "resume" => Command::Resume,
            "q" | "quit" => Command::Quit,
            "af" | "addfilter" => {
                if parts.len() > 1 {
                    Command::AddFilter(parts[1..].join(" "))
                } else {
                    Command::Unknown("Missing filter".to_string())
                }
            }
            "rf" | "removefilter" => {
                if parts.len() > 1 {
                    Command::RemoveFilter(parts[1..].join(" "))
                } else {
                    Command::Unknown("Missing filter".to_string())
                }
            }
            "t" | "threads" => {
                if parts.len() > 1 {
                    if let Ok(n) = parts[1].parse() {
                        Command::ChangeThreads(n)
                    } else {
                        Command::Unknown("Invalid thread count".to_string())
                    }
                } else {
                    Command::Unknown("Missing thread count".to_string())
                }
            }
            _ => Command::Unknown(s.to_string()),
        }
    }
}

/// Handle commands in interactive mode
pub struct CommandHandler;

impl CommandHandler {
    pub fn execute(cmd: Command) -> bool {
        match cmd {
            Command::Help => {
                Self::print_help();
                true
            }
            Command::Stats => {
                println!("Stats command - implement with statistics module");
                true
            }
            Command::Pause => {
                println!("⏸  Pausing...");
                true
            }
            Command::Resume => {
                println!("▶  Resuming...");
                true
            }
            Command::Quit => {
                println!("Quitting...");
                false
            }
            Command::AddFilter(filter) => {
                println!("Adding filter: {}", filter);
                true
            }
            Command::RemoveFilter(filter) => {
                println!("Removing filter: {}", filter);
                true
            }
            Command::ChangeThreads(n) => {
                println!("Changing threads to: {}", n);
                true
            }
            Command::Unknown(msg) => {
                println!("Unknown command: {}. Type 'help' for available commands.", msg);
                true
            }
        }
    }
    
    fn print_help() {
        println!("\n{}", "═".repeat(70));
        println!("FuzzBlitz Interactive Commands:");
        println!("{}", "─".repeat(70));
        println!("  {:20} - {}", "h, help", "Show this help message");
        println!("  {:20} - {}", "s, stats", "Show current scan statistics");
        println!("  {:20} - {}", "p, pause", "Pause the scan");
        println!("  {:20} - {}", "r, resume", "Resume the scan");
        println!("  {:20} - {}", "af <filter>, addfilter", "Add a runtime filter");
        println!("  {:20} - {}", "rf <filter>, removefilter", "Remove a runtime filter");
        println!("  {:20} - {}", "t <num>, threads", "Change thread count");
        println!("  {:20} - {}", "q, quit", "Quit the scan");
        println!("{}", "═".repeat(70));
    }
}
