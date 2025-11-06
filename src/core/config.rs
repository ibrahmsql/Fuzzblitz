#![allow(dead_code)]
use crate::cli::ProgramArgs;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FuzzConfig {
    pub url: String,
    pub method: String,
    pub threads: usize,
    pub timeout: Duration,
    pub follow_redirects: bool,
    pub ignore_body: bool,
    pub delay_min_ms: Option<u64>,
    pub delay_max_ms: Option<u64>,
    pub rate_limit: Option<u64>,
    pub max_time: Option<u64>,
    pub colorize: bool,
    pub verbose: bool,
    pub silent: bool,
    pub json_output: bool,
}

impl FuzzConfig {
    pub fn from_args(args: &ProgramArgs) -> Self {
        let (delay_min, delay_max) = parse_delay(&args.delay);
        
        Self {
            url: args.url.clone().unwrap_or_default(),
            method: args.method.clone(),
            threads: args.threads,
            timeout: Duration::from_secs(args.timeout),
            follow_redirects: args.follow_redirects,
            ignore_body: args.ignore_body,
            delay_min_ms: delay_min,
            delay_max_ms: delay_max,
            rate_limit: if args.rate > 0 { Some(args.rate) } else { None },
            max_time: if args.maxtime > 0 { Some(args.maxtime) } else { None },
            colorize: args.colorize,
            verbose: args.verbose,
            silent: args.silent,
            json_output: args.json_output,
        }
    }
}

fn parse_delay(delay_str: &Option<String>) -> (Option<u64>, Option<u64>) {
    match delay_str {
        None => (None, None),
        Some(s) => {
            if s.contains('-') {
                let parts: Vec<&str> = s.split('-').collect();
                if parts.len() == 2 {
                    let min = (parts[0].parse::<f64>().unwrap_or(0.0) * 1000.0) as u64;
                    let max = (parts[1].parse::<f64>().unwrap_or(0.0) * 1000.0) as u64;
                    (Some(min), Some(max))
                } else {
                    (None, None)
                }
            } else {
                let ms = (s.parse::<f64>().unwrap_or(0.0) * 1000.0) as u64;
                (Some(ms), Some(ms))
            }
        }
    }
}
