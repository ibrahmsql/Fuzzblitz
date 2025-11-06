#![allow(dead_code)]
use colored::Colorize;
use crate::cli::ProgramArgs;

pub fn print_banner(_args: &ProgramArgs) {
    println!();
    println!("{}", "_______________________________________________________________".bright_yellow());
    println!();
    println!("  {}", r#"  _____              ______ _ _ _       "#.bright_cyan().bold());
    println!("  {}", r#" |  ___|            |  _  \ (_) |      "#.bright_cyan().bold());
    println!("  {}", r#" | |__ _   _ _____ _| |_| / |_| |_ ____"#.bright_cyan().bold());
    println!("  {}", r#" |  __| | | |_  / |_  ____|/ | __/_  /"#.bright_cyan().bold());
    println!("  {}", r#" | |  | |_| |/ /| |_| |_| | | |_ / / "#.bright_cyan().bold());
    println!("  {}", r#" \_|   \__,_/___|\__|_____/_|\__/___|"#.bright_cyan().bold());
    println!();
    println!("           {}  {}  {}", 
        "⚡".bright_yellow(),
        "Lightning Fast Web Fuzzer".bright_white().bold(),
        "⚡".bright_yellow()
    );
    println!();
    println!("  {} {}  {} {}  {} {}", 
        "Speed".bright_red().bold(),
        "•".bright_white(),
        "Power".bright_yellow().bold(),
        "•".bright_white(),
        "Precision".bright_green().bold(),
        "•".bright_white()
    );
    println!();
    println!("  {} {}      {} {}", 
        "v2.0.0".bright_white().bold(),
        "|".bright_white(),
        "by".bright_white(),
        "@ibrahimsql".bright_cyan()
    );
    println!("{}", "_______________________________________________________________".bright_yellow());
    println!();
}

pub fn print_config_summary(args: &ProgramArgs) {
    if let Some(ref url) = args.url {
        println!(" :: {} : {}", "Target URL".bright_blue(), url);
    }
    println!(" :: {} : {}", "HTTP Method".bright_blue(), args.method);
    println!(" :: {} : {}", "Threads".bright_blue(), args.threads);
    println!(" :: {} : {}", "Timeout".bright_blue(), format!("{}s", args.timeout));
    
    if args.follow_redirects {
        println!(" :: {} : {}", "Follow Redirects".bright_blue(), "true");
    }
    
    if let Some(ref proxy) = args.proxy {
        println!(" :: {} : {}", "Proxy".bright_blue(), proxy);
    }
    
    if args.http2 {
        println!(" :: {} : {}", "HTTP/2".bright_blue(), "enabled");
    }
    
    if args.recursion {
        println!(" :: {} : {}", "Recursion".bright_blue(), format!("enabled (depth: {})", args.recursion_depth));
    }
    
    println!();
}
