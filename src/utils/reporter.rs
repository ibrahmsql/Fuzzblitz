use crate::output::FuzzResult;
use crate::core::Statistics;
use colored::Colorize;
use std::collections::HashMap;

pub struct Reporter {
    colorize: bool,
}

impl Reporter {
    pub fn new(colorize: bool) -> Self {
        Self { colorize }
    }
    
    pub fn print_banner(&self, config: &str) {
        if self.colorize {
            println!("{}", "═".repeat(70).cyan());
            println!("{}", "  RustFuzz v2.0 - Advanced Web Fuzzer".cyan().bold());
            println!("{}", "  FFUF Enhanced + URX Features".cyan());
            println!("{}", "═".repeat(70).cyan());
        } else {
            println!("{}", "═".repeat(70));
            println!("  RustFuzz v2.0 - Advanced Web Fuzzer");
            println!("  FFUF Enhanced + URX Features");
            println!("{}", "═".repeat(70));
        }
        println!("{}", config);
        println!();
    }
    
    pub fn print_result(&self, result: &FuzzResult, verbose: bool) {
        let status_str = self.colorize_status(result.status_code);
        
        if verbose {
            println!(
                "[{}] {} | Size: {} | Lines: {} | Words: {} | Time: {}ms | {}",
                status_str,
                result.url,
                result.body_length,
                result.lines,
                result.words,
                result.response_time_ms,
                result.fuzz_word
            );
        } else {
            println!(
                "[{}] {} | Size: {} | Time: {}ms",
                status_str,
                result.url,
                result.body_length,
                result.response_time_ms
            );
        }
    }
    
    pub fn print_summary(&self, stats: &Statistics, results_count: usize) {
        println!();
        if self.colorize {
            println!("{}", "═".repeat(70).green());
            println!("{}", "Scan Complete".green().bold());
            println!("{}", "═".repeat(70).green());
        } else {
            println!("{}", "═".repeat(70));
            println!("Scan Complete");
            println!("{}", "═".repeat(70));
        }
        
        println!("Total Requests:    {}", stats.total());
        println!("Completed:         {}", stats.completed());
        println!("Matched:           {} ({:.2}%)", stats.matched(), stats.match_rate());
        println!("Errors:            {} ({:.2}%)", stats.errors(), stats.error_rate());
        println!("Time Elapsed:      {}s", stats.elapsed_secs());
        println!("Requests/Second:   {:.2}", stats.req_per_sec());
        println!("Data Sent:         {:.2} MB", stats.bytes_sent_mb());
        println!("Data Received:     {:.2} MB", stats.bytes_received_mb());
        println!("{}", "═".repeat(70));
    }
    
    pub fn print_error(&self, message: &str) {
        if self.colorize {
            eprintln!("{} {}", "✗".red().bold(), message.red());
        } else {
            eprintln!("Error: {}", message);
        }
    }
    
    pub fn print_warning(&self, message: &str) {
        if self.colorize {
            println!("{} {}", "⚠".yellow().bold(), message.yellow());
        } else {
            println!("Warning: {}", message);
        }
    }
    
    pub fn print_info(&self, message: &str) {
        if self.colorize {
            println!("{} {}", "ℹ".blue().bold(), message);
        } else {
            println!("Info: {}", message);
        }
    }
    
    pub fn print_success(&self, message: &str) {
        if self.colorize {
            println!("{} {}", "✓".green().bold(), message.green());
        } else {
            println!("Success: {}", message);
        }
    }
    
    fn colorize_status(&self, code: u16) -> String {
        if !self.colorize {
            return code.to_string();
        }
        
        match code {
            200..=299 => format!("{}", code).green().to_string(),
            300..=399 => format!("{}", code).blue().to_string(),
            400..=499 => format!("{}", code).yellow().to_string(),
            _ => format!("{}", code).red().to_string(),
        }
    }
    
    pub fn print_status_distribution(&self, results: &[FuzzResult]) {
        let mut distribution: HashMap<u16, usize> = HashMap::new();
        
        for result in results {
            *distribution.entry(result.status_code).or_insert(0) += 1;
        }
        
        println!("\nStatus Code Distribution:");
        println!("{}", "-".repeat(30));
        
        let mut codes: Vec<_> = distribution.keys().collect();
        codes.sort();
        
        for code in codes {
            let count = distribution[code];
            let bar_length = (count as f64 / results.len() as f64 * 20.0) as usize;
            let bar = "█".repeat(bar_length);
            
            if self.colorize {
                println!("  {} : {} {}", 
                    self.colorize_status(*code),
                    count,
                    bar
                );
            } else {
                println!("  {} : {} {}", code, count, bar);
            }
        }
    }
    
    pub fn print_size_distribution(&self, results: &[FuzzResult]) {
        let mut sizes: Vec<_> = results.iter().map(|r| r.body_length).collect();
        sizes.sort();
        
        if sizes.is_empty() {
            return;
        }
        
        let min = sizes[0];
        let max = sizes[sizes.len() - 1];
        let avg = sizes.iter().sum::<usize>() / sizes.len();
        
        println!("\nResponse Size Statistics:");
        println!("{}", "-".repeat(30));
        println!("  Min:     {} bytes", min);
        println!("  Max:     {} bytes", max);
        println!("  Average: {} bytes", avg);
    }
}

pub fn create_config_summary(args: &crate::cli::ProgramArgs) -> String {
    let mut lines = Vec::new();
    
    if let Some(ref url) = args.url {
        lines.push(format!("Target:     {}", url));
    }
    
    lines.push(format!("Method:     {}", args.method));
    lines.push(format!("Threads:    {}", args.threads));
    lines.push(format!("Timeout:    {}s", args.timeout));
    
    if !args.headers.is_empty() {
        lines.push(format!("Headers:    {} custom", args.headers.len()));
    }
    
    if args.proxy.is_some() {
        lines.push("Proxy:      Enabled".to_string());
    }
    
    if !args.wordlist.is_empty() {
        lines.push(format!("Wordlists:  {}", args.wordlist.len()));
    }
    
    lines.join("\n")
}
