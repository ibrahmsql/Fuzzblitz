use std::collections::HashMap;

/// Baseline analyzer for calibration
pub struct BaselineAnalyzer {
    samples: Vec<BaselineSample>,
}

#[derive(Debug, Clone)]
pub struct BaselineSample {
    pub status_code: u16,
    pub size: usize,
    pub lines: usize,
    pub words: usize,
    pub time_ms: u64,
}

#[derive(Debug, Clone)]
pub struct BaselineStats {
    pub avg_size: f64,
    pub avg_lines: f64,
    pub avg_words: f64,
    pub avg_time: f64,
    pub common_status: u16,
    pub size_stddev: f64,
}

impl BaselineAnalyzer {
    pub fn new() -> Self {
        Self {
            samples: Vec::new(),
        }
    }
    
    /// Add a sample to baseline
    pub fn add_sample(&mut self, sample: BaselineSample) {
        self.samples.push(sample);
    }
    
    /// Calculate baseline statistics
    pub fn calculate_stats(&self) -> Option<BaselineStats> {
        if self.samples.is_empty() {
            return None;
        }
        
        let n = self.samples.len() as f64;
        
        // Calculate averages
        let avg_size = self.samples.iter().map(|s| s.size as f64).sum::<f64>() / n;
        let avg_lines = self.samples.iter().map(|s| s.lines as f64).sum::<f64>() / n;
        let avg_words = self.samples.iter().map(|s| s.words as f64).sum::<f64>() / n;
        let avg_time = self.samples.iter().map(|s| s.time_ms as f64).sum::<f64>() / n;
        
        // Find most common status code
        let mut status_counts: HashMap<u16, usize> = HashMap::new();
        for sample in &self.samples {
            *status_counts.entry(sample.status_code).or_insert(0) += 1;
        }
        let common_status = *status_counts.iter()
            .max_by_key(|(_, count)| *count)
            .map(|(status, _)| status)
            .unwrap_or(&200);
        
        // Calculate standard deviation for size
        let variance = self.samples.iter()
            .map(|s| {
                let diff = s.size as f64 - avg_size;
                diff * diff
            })
            .sum::<f64>() / n;
        let size_stddev = variance.sqrt();
        
        Some(BaselineStats {
            avg_size,
            avg_lines,
            avg_words,
            avg_time,
            common_status,
            size_stddev,
        })
    }
    
    /// Check if a response deviates from baseline
    pub fn is_deviation(&self, sample: &BaselineSample, threshold: f64) -> bool {
        if let Some(stats) = self.calculate_stats() {
            let size_diff = (sample.size as f64 - stats.avg_size).abs();
            let deviation = size_diff / stats.size_stddev;
            deviation > threshold
        } else {
            false
        }
    }
    
    /// Get sample count
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }
    
    /// Clear all samples
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

impl Default for BaselineAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
