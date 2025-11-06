use super::parser::{ConfigFile, HttpConfig, FuzzingConfig, FilteringConfig, OutputConfig};

pub struct DefaultConfig;

impl DefaultConfig {
    /// Create default configuration
    pub fn create() -> ConfigFile {
        ConfigFile {
            http: HttpConfig {
                method: Some("GET".to_string()),
                headers: None,
                cookies: None,
                data: None,
                proxy: None,
                timeout: Some(10),
                follow_redirects: Some(false),
                http2: Some(false),
            },
            fuzzing: FuzzingConfig {
                wordlists: None,
                mode: Some("clusterbomb".to_string()),
                threads: Some(40),
                rate: Some(0),
                delay: None,
                extensions: None,
                encoders: None,
                recursion: Some(false),
                recursion_depth: Some(0),
            },
            filtering: FilteringConfig {
                match_codes: Some("200-299,301,302,307,401,403,405,500".to_string()),
                match_size: None,
                match_words: None,
                match_lines: None,
                match_regexp: None,
                filter_codes: None,
                filter_size: None,
                filter_words: None,
                filter_lines: None,
                filter_regexp: None,
            },
            output: OutputConfig {
                output_file: None,
                output_format: Some("json".to_string()),
                output_dir: None,
                colorize: Some(false),
                verbose: Some(false),
                silent: Some(false),
            },
        }
    }
    
    /// Merge user config with defaults
    pub fn merge(user_config: ConfigFile) -> ConfigFile {
        let default = Self::create();
        
        ConfigFile {
            http: HttpConfig {
                method: user_config.http.method.or(default.http.method),
                headers: user_config.http.headers.or(default.http.headers),
                cookies: user_config.http.cookies.or(default.http.cookies),
                data: user_config.http.data.or(default.http.data),
                proxy: user_config.http.proxy.or(default.http.proxy),
                timeout: user_config.http.timeout.or(default.http.timeout),
                follow_redirects: user_config.http.follow_redirects.or(default.http.follow_redirects),
                http2: user_config.http.http2.or(default.http.http2),
            },
            fuzzing: FuzzingConfig {
                wordlists: user_config.fuzzing.wordlists.or(default.fuzzing.wordlists),
                mode: user_config.fuzzing.mode.or(default.fuzzing.mode),
                threads: user_config.fuzzing.threads.or(default.fuzzing.threads),
                rate: user_config.fuzzing.rate.or(default.fuzzing.rate),
                delay: user_config.fuzzing.delay.or(default.fuzzing.delay),
                extensions: user_config.fuzzing.extensions.or(default.fuzzing.extensions),
                encoders: user_config.fuzzing.encoders.or(default.fuzzing.encoders),
                recursion: user_config.fuzzing.recursion.or(default.fuzzing.recursion),
                recursion_depth: user_config.fuzzing.recursion_depth.or(default.fuzzing.recursion_depth),
            },
            filtering: FilteringConfig {
                match_codes: user_config.filtering.match_codes.or(default.filtering.match_codes),
                match_size: user_config.filtering.match_size.or(default.filtering.match_size),
                match_words: user_config.filtering.match_words.or(default.filtering.match_words),
                match_lines: user_config.filtering.match_lines.or(default.filtering.match_lines),
                match_regexp: user_config.filtering.match_regexp.or(default.filtering.match_regexp),
                filter_codes: user_config.filtering.filter_codes.or(default.filtering.filter_codes),
                filter_size: user_config.filtering.filter_size.or(default.filtering.filter_size),
                filter_words: user_config.filtering.filter_words.or(default.filtering.filter_words),
                filter_lines: user_config.filtering.filter_lines.or(default.filtering.filter_lines),
                filter_regexp: user_config.filtering.filter_regexp.or(default.filtering.filter_regexp),
            },
            output: OutputConfig {
                output_file: user_config.output.output_file.or(default.output.output_file),
                output_format: user_config.output.output_format.or(default.output.output_format),
                output_dir: user_config.output.output_dir.or(default.output.output_dir),
                colorize: user_config.output.colorize.or(default.output.colorize),
                verbose: user_config.output.verbose.or(default.output.verbose),
                silent: user_config.output.silent.or(default.output.silent),
            },
        }
    }
}
