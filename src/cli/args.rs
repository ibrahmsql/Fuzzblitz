#![allow(dead_code)]
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(
    author="ibrahimsql", 
    version="v2.0.0", 
    about="⚡ FuzzBlitz - Lightning Fast Web Fuzzer", 
    long_about="A blazingly fast web fuzzer written in Rust.\nFeatures: Multi-threaded, smart filtering, multiple output formats, and more.\n\nGitHub: https://github.com/ibrahimsql/fuzzblitz",
)]
pub struct ProgramArgs {
    // ============ HTTP OPTIONS ============
    #[clap(
        short='u', 
        long="url", 
        help="Target URL (use FUZZ keyword as placeholder)"
    )]
    pub url: Option<String>,

    #[clap(
        short='X', 
        long="method", 
        default_value="GET",
        help="HTTP method to use (GET, POST, PUT, DELETE, etc.)"
    )]
    pub method: String,

    #[clap(
        short='H', 
        long="header", 
        help="Header 'Name: Value'. Multiple -H flags accepted"
    )]
    pub headers: Vec<String>,

    #[clap(
        short='d', 
        long="data", 
        help="POST data"
    )]
    pub data: Option<String>,

    #[clap(
        short='b', 
        long="cookie", 
        help="Cookie data 'NAME1=VALUE1; NAME2=VALUE2'"
    )]
    pub cookies: Option<String>,

    #[clap(
        long="timeout", 
        default_value="10",
        help="HTTP request timeout in seconds"
    )]
    pub timeout: u64,

    #[clap(
        short='r', 
        long="follow-redirects",
        help="Follow redirects"
    )]
    pub follow_redirects: bool,

    #[clap(
        short='x', 
        long="proxy", 
        help="Proxy URL (HTTP or SOCKS5). Example: http://127.0.0.1:8080"
    )]
    pub proxy: Option<String>,

    #[clap(
        long="http2",
        help="Use HTTP/2 protocol"
    )]
    pub http2: bool,

    #[clap(
        long="recursion",
        help="Scan recursively"
    )]
    pub recursion: bool,

    #[clap(
        long="recursion-depth",
        default_value="0",
        help="Maximum recursion depth"
    )]
    pub recursion_depth: usize,

    #[clap(
        long="ignore-body",
        help="Do not fetch response content"
    )]
    pub ignore_body: bool,

    // ============ GENERAL OPTIONS ============
    #[clap(
        short='t', 
        long="threads", 
        default_value="40",
        help="Number of concurrent threads"
    )]
    pub threads: usize,

    #[clap(
        short='c', 
        long="color",
        help="Colorize output"
    )]
    pub colorize: bool,

    #[clap(
        short='v', 
        long="verbose",
        help="Verbose output"
    )]
    pub verbose: bool,

    #[clap(
        short='s', 
        long="silent",
        help="Silent mode - no extra info"
    )]
    pub silent: bool,

    #[clap(
        short='p', 
        long="delay", 
        help="Delay between requests in seconds (e.g., 0.1 or 0.1-2.0 for random)"
    )]
    pub delay: Option<String>,

    #[clap(
        long="rate", 
        default_value="0",
        help="Rate of requests per second (0 = unlimited)"
    )]
    pub rate: u64,

    #[clap(
        long="maxtime", 
        default_value="0",
        help="Maximum running time in seconds (0 = unlimited)"
    )]
    pub maxtime: u64,

    #[clap(
        long="json",
        help="JSON output format"
    )]
    pub json_output: bool,

    #[clap(
        long="ac",
        help="Automatically calibrate filtering options"
    )]
    pub auto_calibrate: bool,

    // ============ MATCHER OPTIONS ============
    #[clap(
        long="mc", 
        help="Match HTTP status codes (comma-separated or 'all')",
        default_value="all"
    )]
    pub match_codes: String,

    #[clap(
        long="ml", 
        help="Match amount of lines in response"
    )]
    pub match_lines: Vec<String>,

    #[clap(
        long="mr", 
        help="Match regexp pattern"
    )]
    pub match_regexp: Vec<String>,

    #[clap(
        long="ms", 
        help="Match HTTP response size"
    )]
    pub match_size: Vec<String>,

    #[clap(
        long="mw", 
        help="Match amount of words in response"
    )]
    pub match_words: Vec<String>,

    #[clap(
        long="mt", 
        help="Match response time (e.g., >100 or <100 ms)"
    )]
    pub match_time: Option<String>,

    #[clap(
        long="mmode", 
        default_value="or",
        help="Matcher set operator: 'and' or 'or'"
    )]
    pub match_mode: String,

    // ============ FILTER OPTIONS ============
    #[clap(
        long="fc", 
        help="Filter HTTP status codes (comma-separated)"
    )]
    pub filter_codes: Vec<String>,

    #[clap(
        long="fl", 
        help="Filter by amount of lines in response"
    )]
    pub filter_lines: Vec<String>,

    #[clap(
        long="fr", 
        help="Filter regexp pattern"
    )]
    pub filter_regexp: Vec<String>,

    #[clap(
        long="fs", 
        help="Filter HTTP response size"
    )]
    pub filter_size: Vec<String>,

    #[clap(
        long="fw", 
        help="Filter by amount of words in response"
    )]
    pub filter_words: Vec<String>,

    #[clap(
        long="ft", 
        help="Filter by response time (e.g., >100 or <100 ms)"
    )]
    pub filter_time: Option<String>,

    #[clap(
        long="fmode", 
        default_value="or",
        help="Filter set operator: 'and' or 'or'"
    )]
    pub filter_mode: String,

    // ============ INPUT OPTIONS ============
    #[clap(
        short='w', 
        long="wordlist", 
        help="Wordlist file path (can specify keyword: /path/to/wordlist:KEYWORD)"
    )]
    pub wordlist: Vec<String>,

    #[clap(
        short='e', 
        long="extensions", 
        help="Comma-separated list of extensions (e.g., .php,.html,.bak)"
    )]
    pub extensions: Option<String>,

    #[clap(
        long="mode", 
        default_value="clusterbomb",
        help="Multi-wordlist operation mode: clusterbomb, pitchfork, sniper"
    )]
    pub mode: String,

    #[clap(
        long="enc", 
        help="Encoders for keywords (e.g., 'FUZZ:urlencode,b64encode')"
    )]
    pub encoders: Vec<String>,

    #[clap(
        long="ic",
        help="Ignore wordlist comments"
    )]
    pub ignore_comments: bool,

    // ============ OUTPUT OPTIONS ============
    #[clap(
        short='o', 
        long="output", 
        help="Write output to file"
    )]
    pub output: Option<String>,

    #[clap(
        long="of", 
        default_value="json",
        help="Output file format: json, html, csv, md, all"
    )]
    pub output_format: String,

    #[clap(
        long="od", 
        help="Directory to store matched results"
    )]
    pub output_dir: Option<String>,

    // ============ LEGACY COMPATIBILITY ============
    #[clap(
        long="status-code", 
        help="(Legacy) Specify status codes to show"
    )]
    pub status_codes: Vec<String>,
    
    #[clap(
        long="exclude-sc", 
        help="(Legacy) Specify status codes to exclude"
    )]
    pub exclude_status_codes: Vec<String>,
}
