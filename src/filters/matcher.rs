use regex::Regex;
use crate::cli::ProgramArgs;
use super::response::FuzzResponse;

pub struct MatcherFilter {
    // Matchers
    match_codes: Vec<StatusCodeMatcher>,
    match_lines: Vec<RangeMatcher>,
    match_size: Vec<RangeMatcher>,
    match_words: Vec<RangeMatcher>,
    match_regexp: Vec<Regex>,
    match_time: Option<TimeMatcher>,
    match_mode: MatchMode,
    
    // Filters
    filter_codes: Vec<StatusCodeMatcher>,
    filter_lines: Vec<RangeMatcher>,
    filter_size: Vec<RangeMatcher>,
    filter_words: Vec<RangeMatcher>,
    filter_regexp: Vec<Regex>,
    filter_time: Option<TimeMatcher>,
    filter_mode: MatchMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MatchMode {
    And,
    Or,
}

impl From<&str> for MatchMode {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "and" => MatchMode::And,
            _ => MatchMode::Or,
        }
    }
}

#[derive(Debug, Clone)]
enum StatusCodeMatcher {
    Exact(u16),
    Range(u16, u16),
    All,
}

impl StatusCodeMatcher {
    fn matches(&self, code: u16) -> bool {
        match self {
            StatusCodeMatcher::Exact(c) => *c == code,
            StatusCodeMatcher::Range(start, end) => code >= *start && code <= *end,
            StatusCodeMatcher::All => true,
        }
    }
}

#[derive(Debug, Clone)]
enum RangeMatcher {
    Exact(usize),
    Range(usize, usize),
}

impl RangeMatcher {
    fn matches(&self, value: usize) -> bool {
        match self {
            RangeMatcher::Exact(v) => *v == value,
            RangeMatcher::Range(start, end) => value >= *start && value <= *end,
        }
    }
}

#[derive(Debug, Clone)]
enum TimeMatcher {
    GreaterThan(i64),
    LessThan(i64),
}

impl TimeMatcher {
    fn matches(&self, time_ms: i64) -> bool {
        match self {
            TimeMatcher::GreaterThan(t) => time_ms > *t,
            TimeMatcher::LessThan(t) => time_ms < *t,
        }
    }
}

impl MatcherFilter {
    pub fn from_args(args: &ProgramArgs) -> Result<Self, String> {
        Ok(Self {
            match_codes: parse_status_codes(&args.match_codes)?,
            match_lines: parse_ranges(&args.match_lines)?,
            match_size: parse_ranges(&args.match_size)?,
            match_words: parse_ranges(&args.match_words)?,
            match_regexp: parse_regexps(&args.match_regexp)?,
            match_time: parse_time_matcher(args.match_time.as_deref())?,
            match_mode: MatchMode::from(args.match_mode.as_str()),
            
            filter_codes: parse_status_codes_list(&args.filter_codes)?,
            filter_lines: parse_ranges(&args.filter_lines)?,
            filter_size: parse_ranges(&args.filter_size)?,
            filter_words: parse_ranges(&args.filter_words)?,
            filter_regexp: parse_regexps(&args.filter_regexp)?,
            filter_time: parse_time_matcher(args.filter_time.as_deref())?,
            filter_mode: MatchMode::from(args.filter_mode.as_str()),
        })
    }
    
    pub fn should_show(&self, response: &FuzzResponse) -> bool {
        // First check filters (exclusions)
        if self.apply_filters(response) {
            return false;
        }
        
        // Then check matchers (inclusions)
        self.apply_matchers(response)
    }
    
    fn apply_matchers(&self, response: &FuzzResponse) -> bool {
        let mut results = Vec::new();
        
        // Status code matching
        if !self.match_codes.is_empty() {
            let matches = self.match_codes.iter()
                .any(|m| m.matches(response.status_code));
            results.push(matches);
        }
        
        // Lines matching
        if !self.match_lines.is_empty() {
            let matches = self.match_lines.iter()
                .any(|m| m.matches(response.lines));
            results.push(matches);
        }
        
        // Size matching
        if !self.match_size.is_empty() {
            let matches = self.match_size.iter()
                .any(|m| m.matches(response.body_length));
            results.push(matches);
        }
        
        // Words matching
        if !self.match_words.is_empty() {
            let matches = self.match_words.iter()
                .any(|m| m.matches(response.words));
            results.push(matches);
        }
        
        // Regexp matching
        if !self.match_regexp.is_empty() {
            let matches = self.match_regexp.iter()
                .any(|re| re.is_match(&response.body));
            results.push(matches);
        }
        
        // Time matching
        if let Some(ref time_matcher) = self.match_time {
            results.push(time_matcher.matches(response.response_time_ms));
        }
        
        // If no matchers specified, show everything
        if results.is_empty() {
            return true;
        }
        
        // Apply match mode
        match self.match_mode {
            MatchMode::And => results.iter().all(|&r| r),
            MatchMode::Or => results.iter().any(|&r| r),
        }
    }
    
    fn apply_filters(&self, response: &FuzzResponse) -> bool {
        let mut results = Vec::new();
        
        // Status code filtering
        if !self.filter_codes.is_empty() {
            let matches = self.filter_codes.iter()
                .any(|m| m.matches(response.status_code));
            results.push(matches);
        }
        
        // Lines filtering
        if !self.filter_lines.is_empty() {
            let matches = self.filter_lines.iter()
                .any(|m| m.matches(response.lines));
            results.push(matches);
        }
        
        // Size filtering
        if !self.filter_size.is_empty() {
            let matches = self.filter_size.iter()
                .any(|m| m.matches(response.body_length));
            results.push(matches);
        }
        
        // Words filtering
        if !self.filter_words.is_empty() {
            let matches = self.filter_words.iter()
                .any(|m| m.matches(response.words));
            results.push(matches);
        }
        
        // Regexp filtering
        if !self.filter_regexp.is_empty() {
            let matches = self.filter_regexp.iter()
                .any(|re| re.is_match(&response.body));
            results.push(matches);
        }
        
        // Time filtering
        if let Some(ref time_matcher) = self.filter_time {
            results.push(time_matcher.matches(response.response_time_ms));
        }
        
        // If no filters specified, don't filter anything
        if results.is_empty() {
            return false;
        }
        
        // Apply filter mode
        match self.filter_mode {
            MatchMode::And => results.iter().all(|&r| r),
            MatchMode::Or => results.iter().any(|&r| r),
        }
    }
}

// Parsing functions
fn parse_status_codes(codes_str: &str) -> Result<Vec<StatusCodeMatcher>, String> {
    if codes_str.to_lowercase() == "all" {
        return Ok(vec![StatusCodeMatcher::All]);
    }
    
    let mut matchers = Vec::new();
    for part in codes_str.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() != 2 {
                return Err(format!("Invalid range: {}", part));
            }
            let start = range[0].parse::<u16>()
                .map_err(|_| format!("Invalid status code: {}", range[0]))?;
            let end = range[1].parse::<u16>()
                .map_err(|_| format!("Invalid status code: {}", range[1]))?;
            matchers.push(StatusCodeMatcher::Range(start, end));
        } else {
            let code = part.parse::<u16>()
                .map_err(|_| format!("Invalid status code: {}", part))?;
            matchers.push(StatusCodeMatcher::Exact(code));
        }
    }
    Ok(matchers)
}

fn parse_status_codes_list(codes: &[String]) -> Result<Vec<StatusCodeMatcher>, String> {
    let mut all_matchers = Vec::new();
    for code_str in codes {
        let mut matchers = parse_status_codes(code_str)?;
        all_matchers.append(&mut matchers);
    }
    Ok(all_matchers)
}

fn parse_ranges(ranges: &[String]) -> Result<Vec<RangeMatcher>, String> {
    let mut matchers = Vec::new();
    for range_str in ranges {
        for part in range_str.split(',') {
            let part = part.trim();
            if part.contains('-') {
                let range: Vec<&str> = part.split('-').collect();
                if range.len() != 2 {
                    return Err(format!("Invalid range: {}", part));
                }
                let start = range[0].parse::<usize>()
                    .map_err(|_| format!("Invalid number: {}", range[0]))?;
                let end = range[1].parse::<usize>()
                    .map_err(|_| format!("Invalid number: {}", range[1]))?;
                matchers.push(RangeMatcher::Range(start, end));
            } else {
                let value = part.parse::<usize>()
                    .map_err(|_| format!("Invalid number: {}", part))?;
                matchers.push(RangeMatcher::Exact(value));
            }
        }
    }
    Ok(matchers)
}

fn parse_regexps(patterns: &[String]) -> Result<Vec<Regex>, String> {
    let mut regexps = Vec::new();
    for pattern in patterns {
        let re = Regex::new(pattern)
            .map_err(|e| format!("Invalid regex '{}': {}", pattern, e))?;
        regexps.push(re);
    }
    Ok(regexps)
}

fn parse_time_matcher(time_str: Option<&str>) -> Result<Option<TimeMatcher>, String> {
    match time_str {
        None => Ok(None),
        Some(s) => {
            let s = s.trim();
            if s.starts_with('>') {
                let value = s[1..].parse::<i64>()
                    .map_err(|_| format!("Invalid time value: {}", s))?;
                Ok(Some(TimeMatcher::GreaterThan(value)))
            } else if s.starts_with('<') {
                let value = s[1..].parse::<i64>()
                    .map_err(|_| format!("Invalid time value: {}", s))?;
                Ok(Some(TimeMatcher::LessThan(value)))
            } else {
                Err(format!("Time matcher must start with > or <: {}", s))
            }
        }
    }
}
