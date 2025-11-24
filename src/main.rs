#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::error::Error;
use std::sync::Arc;
use std::time::Instant;
use std::collections::HashMap;

use clap::Parser;
use tokio::time::{sleep, Duration};
use futures::stream::{self, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use colored::Colorize;

// Module declarations
mod cli;
mod network;
mod filters;
mod input;
mod output;
mod core;
mod utils;
mod interactive;
mod job;
mod scraper;
mod history;
mod config_loader;
mod runner;
mod plugin;
mod analyzer;
mod waf;
mod auth;
mod api;
mod graphql;
mod websocket;
mod session;
mod cache;
mod param_miner;
mod cors;
mod headers;
mod redirect;
mod subdomain;

// Imports from our modules
use cli::{ProgramArgs, print_banner, print_config_summary};
use network::{FuzzClient, build_url, add_extension};
use filters::{MatcherFilter, FuzzResponse};
use input::{Wordlist, PayloadGenerator, FuzzMode, parse_wordlist_spec, parse_encoder_spec, encode_payload, UrlList};
use output::{OutputWriter, OutputFormat, FuzzResult};
use core::{Statistics, RateLimiter};
// Unused import - commented to fix build warnings
// use utils::auto_calibrate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = ProgramArgs::parse();
    
    // Validate arguments - either URL or URLs file required
    if args.url.is_none() && args.urls_file.is_none() {
        eprintln!("{}\n", "Error: Either -u (URL) or -U (URLs file) flag is required".red().bold());
        eprintln!("Examples:");
        eprintln!("  Single URL:    fuzzblitz -u https://example.com/FUZZ -w wordlist.txt");
        eprintln!("  Multiple URLs: fuzzblitz -U urls.txt -w wordlist.txt");
        std::process::exit(1);
    }
    
    if args.url.is_some() && args.urls_file.is_some() {
        eprintln!("{}\n", "Error: Cannot use both -u and -U flags together".red().bold());
        eprintln!("Use either -u for a single URL or -U for a file containing multiple URLs");
        std::process::exit(1);
    }
    
    if args.wordlist.is_empty() {
        eprintln!("{}\n", "Error: -w (wordlist) flag is required".red().bold());
        eprintln!("Example: fuzzblitz -u https://example.com/FUZZ -w wordlist.txt");
        std::process::exit(1);
    }
    
    // Load URLs - either from single URL or from file
    let urls = if let Some(ref url_file) = args.urls_file {
        // Load URLs from file
        match UrlList::from_file(url_file, args.auto_fuzz) {
            Ok(url_list) => {
                if !args.auto_fuzz {
                    // Validate that all URLs contain FUZZ if auto_fuzz is not enabled
                    if let Err(e) = url_list.validate_urls() {
                        eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
                        eprintln!("{}\n", format!("❌ Error: {}", e).red().bold());
                        eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
                        std::process::exit(1);
                    }
                }
                if !args.silent {
                    println!("[+] Loaded {} URLs from {}", url_list.len(), url_file);
                }
                url_list.urls
            },
            Err(e) => {
                eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
                eprintln!("{}\n", format!("❌ Error loading URLs file: {}", e).red().bold());
                eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
                std::process::exit(1);
            }
        }
    } else {
        // Single URL mode
        let url = args.url.as_ref().unwrap();
        
        if !url.contains("FUZZ") {
            eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
            eprintln!("{}\n", "❌ Error: URL must contain the keyword 'FUZZ'".red().bold());
            eprintln!("{}", "The FUZZ keyword is required for fuzzing!".yellow());
            eprintln!("{}", "It will be replaced with each word from the wordlist.\n".yellow());
            eprintln!("{}", "Examples:".bright_cyan().bold());
            eprintln!("  {} fuzzblitz -u https://example.com/{} -w wordlist.txt", "⚡".bright_yellow(), "FUZZ".bright_green().bold());
            eprintln!("  {} fuzzblitz -u https://example.com/api/{}/ -w wordlist.txt", "⚡".bright_yellow(), "FUZZ".bright_green().bold());
            eprintln!("  {} fuzzblitz -u https://example.com/{}/admin -w wordlist.txt", "⚡".bright_yellow(), "FUZZ".bright_green().bold());
            eprintln!("  {} fuzzblitz -u https://example.com/?page={} -w wordlist.txt\n", "⚡".bright_yellow(), "FUZZ".bright_green().bold());
            eprintln!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".red());
            std::process::exit(1);
        }
        
        vec![url.clone()]
    };
    
    // Print banner and config
    if !args.silent {
        print_banner(&args);
        print_config_summary(&args);
    }
    
    // Load wordlists
    let mut wordlists = Vec::new();
    for wordlist_spec in &args.wordlist {
        let (path, keyword) = parse_wordlist_spec(wordlist_spec);
        match Wordlist::from_file(&path, keyword) {
            Ok(wl) => {
                if !args.silent {
                    println!("[+] Loaded {} words for keyword {}", wl.words.len(), wl.keyword);
                }
                wordlists.push(wl);
            },
            Err(e) => {
                eprintln!("Error loading wordlist {}: {}", path, e);
                std::process::exit(1);
            }
        }
    }
    
    // Setup payload generator
    let mode = FuzzMode::from_str(&args.mode);
    let payload_gen = PayloadGenerator::new(wordlists.clone(), mode);
    let total_requests = payload_gen.total_requests();
    
    if !args.silent {
        println!("[+] Total requests: {}", total_requests);
        println!("[+] Mode: {:?}", mode);
        println!("[+] Threads: {}", args.threads);
        println!();
    }
    
    // Setup matcher/filter
    let matcher_filter = Arc::new(MatcherFilter::from_args(&args)?);
    
    // Setup HTTP client
    let http_client = Arc::new(FuzzClient::from_args(&args)?);
    
    // Setup output writer
    let output_format = OutputFormat::from_str(&args.output_format);
    let mut output_writer = OutputWriter::new(args.output.clone(), output_format)?;
    output_writer.write_header()?;
    
    // Setup encoder map
    let mut encoder_map: HashMap<String, Vec<String>> = HashMap::new();
    for enc_spec in &args.encoders {
        let (keyword, encoders) = parse_encoder_spec(enc_spec);
        encoder_map.insert(keyword, encoders);
    }
    let encoder_map = Arc::new(encoder_map);
    
    // Setup extensions
    let extensions: Vec<String> = if let Some(ref ext_str) = args.extensions {
        ext_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        vec![]
    };
    
    // Setup delay range
    let delay_range = parse_delay(&args.delay);
    
    // Global statistics across all URLs
    let mut total_matched = 0;
    let global_start = Instant::now();
    
    // Process each URL
    for (url_idx, url) in urls.iter().enumerate() {
        if urls.len() > 1 && !args.silent {
            println!();
            println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
            println!("{} URL {}/{}: {}", "⚡".bright_yellow(), url_idx + 1, urls.len(), url.bright_white());
            println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_cyan());
        }
        
        // Setup progress bar for this URL
        let progress = if !args.silent && !args.json_output {
            let pb = ProgressBar::new(total_requests as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({per_sec}) | Matched: {msg}")
                    .unwrap()
                    .progress_chars("#>-")
            );
            Some(pb)
        } else {
            None
        };
        
        // Setup rate limiter
        let rate_limiter = Arc::new(RateLimiter::new(args.threads, if args.rate > 0 { Some(args.rate) } else { None }));
        
        // Statistics for this URL
        let stats = Arc::new(Statistics::new(total_requests));
        
        // Prepare payloads
        let mut all_payloads = Vec::new();
        for payload_map in payload_gen.clone() {
            all_payloads.push(payload_map);
        }
        
        // Process requests concurrently
        let results = stream::iter(all_payloads)
            .map(|payload_map| {
                let url = url.clone();
                let http_client = Arc::clone(&http_client);
                let matcher_filter = Arc::clone(&matcher_filter);
                let rate_limiter = rate_limiter.clone_limiter();
                let encoder_map = Arc::clone(&encoder_map);
                let args = args.clone();
                let extensions = extensions.clone();
                let delay_range = delay_range;
                let stats_clone = Arc::clone(&stats);
                
                async move {
                    let _guard = rate_limiter.acquire().await;
                    
                    // Apply delay if specified
                    if let Some((min_ms, max_ms)) = delay_range {
                        let delay_ms = if min_ms == max_ms {
                            min_ms
                        } else {
                            rand::thread_rng().gen_range(min_ms..=max_ms)
                        };
                        sleep(Duration::from_millis(delay_ms)).await;
                    }
                    
                    // Build replacements with encoding
                    let mut replacements = Vec::new();
                    for (keyword, value) in &payload_map {
                        let encoded_value = if let Some(encoders) = encoder_map.get(keyword.as_str()) {
                            encode_payload(value, encoders)
                        } else {
                            value.to_string()
                        };
                        replacements.push((keyword.clone(), encoded_value));
                    }
                    
                    // Try extensions if specified
                    let urls_to_try = if extensions.is_empty() {
                        vec![build_url(&url, &replacements)]
                    } else {
                        let base_url = build_url(&url, &replacements);
                        let mut urls = vec![base_url.clone()];
                        for ext in &extensions {
                            urls.push(add_extension(&base_url, ext));
                        }
                        urls
                    };
                    
                    let mut results = Vec::new();
                    for test_url in urls_to_try {
                        let start = Instant::now();
                        
                        match http_client.send_request(&test_url, args.ignore_body).await {
                            Ok((status_code, body)) => {
                                let response_time_ms = start.elapsed().as_millis() as i64;
                                
                                let fuzz_response = FuzzResponse::new(
                                    status_code,
                                    body,
                                    response_time_ms
                                );
                                
                                if matcher_filter.should_show(&fuzz_response) {
                                    let keyword_str = payload_map.iter()
                                        .map(|(k, v)| format!("{}={}", k, v))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    
                                    results.push(FuzzResult::new(
                                        keyword_str,
                                        test_url,
                                        fuzz_response.status_code,
                                        fuzz_response.body_length,
                                        fuzz_response.lines,
                                        fuzz_response.words,
                                        response_time_ms,
                                    ));
                                }
                            },
                            Err(_) => {
                                // Silently skip errors or log if verbose
                            }
                        }
                    }
                    
                    results
                }
            })
            .buffer_unordered(args.threads)
            .collect::<Vec<_>>()
            .await;
        
        // Process results for this URL
        let mut matched_count = 0;
        for result_vec in results {
            stats.increment_completed();
            
            for result in result_vec {
                matched_count += 1;
                stats.increment_matched();
                
                // Print result
                if !args.silent && !args.json_output {
                    // Clear progress bar line before printing result
                    if progress.is_some() {
                        print!("\r{}\r", " ".repeat(100));
                    }
                    print_result(&result, args.colorize, args.verbose);
                    use std::io::{self, Write};
                    io::stdout().flush().unwrap();
                } else if args.json_output {
                    let json = serde_json::to_string(&result)?;
                    println!("{}", json);
                }
                
                // Save to file
                output_writer.write_result(&result)?;
            }
            
            // Update progress
            if let Some(ref pb) = progress {
                pb.set_message(format!("{}", matched_count));
                pb.inc(1);
            }
        }
        
        if let Some(pb) = progress {
            pb.finish_with_message(format!("{}", matched_count));
        }
        
        // Print summary for this URL
        if urls.len() > 1 && !args.silent {
            println!();
            println!(" :: {} : {} matched ({}% match rate) for {}", 
                "Results".bright_blue(), 
                stats.matched(),
                stats.match_rate() as u64,
                url.bright_white()
            );
        }
        
        total_matched += matched_count;
    }
    
    // Write footer
    output_writer.write_footer()?;
    
    // Print final summary
    if !args.silent {
        println!();
        println!("{}", "________________________________________________".bright_white());
        println!();
        if urls.len() > 1 {
            println!(" :: {} : Fuzzed {} URLs", "Summary".bright_green(), urls.len());
        }
        let elapsed_secs = global_start.elapsed().as_secs();
        println!(" :: {} : {} total matched across all URLs", 
            "Results".bright_blue(), 
            total_matched
        );
        println!(" :: {} : Duration: [0:00:{:02}]", "Time".bright_blue(), elapsed_secs);
        println!();
        println!("{}", "________________________________________________".bright_white());
    }
    
    Ok(())
}


fn print_result(result: &FuzzResult, colorize: bool, verbose: bool) {
    let status_str = if colorize {
        match result.status_code {
            200..=299 => format!("{}", result.status_code).green().bold(),
            300..=399 => format!("{}", result.status_code).cyan().bold(),
            400..=499 => format!("{}", result.status_code).yellow().bold(),
            500..=599 => format!("{}", result.status_code).red().bold(),
            _ => format!("{}", result.status_code).white().bold(),
        }
    } else {
        format!("{}", result.status_code).normal()
    };
    
    // ffuf-style output format
    if verbose {
        println!(
            "{:<15} [Status: {}, Size: {}, Words: {}, Lines: {}, Duration: {}ms]",
            result.fuzz_word.bright_white(),
            status_str,
            result.body_length.to_string().bright_cyan(),
            result.words.to_string().bright_yellow(),
            result.lines.to_string().bright_green(),
            result.response_time_ms.to_string().bright_magenta()
        );
    } else {
        println!(
            "{:<15} [Status: {}, Size: {}, Words: {}, Lines: {}]",
            result.fuzz_word.bright_white(),
            status_str,
            result.body_length.to_string().bright_cyan(),
            result.words.to_string().bright_yellow(),
            result.lines.to_string().bright_green()
        );
    }
}

fn parse_delay(delay_str: &Option<String>) -> Option<(u64, u64)> {
    delay_str.as_ref().and_then(|s| {
        if s.contains('-') {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() == 2 {
                let min = (parts[0].parse::<f64>().ok()? * 1000.0) as u64;
                let max = (parts[1].parse::<f64>().ok()? * 1000.0) as u64;
                Some((min, max))
            } else {
                None
            }
        } else {
            let ms = (s.parse::<f64>().ok()? * 1000.0) as u64;
            Some((ms, ms))
        }
    })
}

fn print_version() {
    println!("RustFuzz v1.0");
}
