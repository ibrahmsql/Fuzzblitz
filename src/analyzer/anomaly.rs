#![allow(dead_code)]
/// Detect anomalies in responses
pub struct AnomalyDetector {
    baseline_size: Option<usize>,
    baseline_time: Option<u64>,
    size_threshold: f64,
    time_threshold: f64,
}

impl AnomalyDetector {
    pub fn new() -> Self {
        Self {
            baseline_size: None,
            baseline_time: None,
            size_threshold: 0.3,  // 30% deviation
            time_threshold: 2.0,  // 2x baseline
        }
    }
    
    pub fn with_thresholds(mut self, size_threshold: f64, time_threshold: f64) -> Self {
        self.size_threshold = size_threshold;
        self.time_threshold = time_threshold;
        self
    }
    
    /// Set baseline from multiple samples
    pub fn set_baseline(&mut self, sizes: &[usize], times: &[u64]) {
        if !sizes.is_empty() {
            let avg_size: usize = sizes.iter().sum::<usize>() / sizes.len();
            self.baseline_size = Some(avg_size);
        }
        
        if !times.is_empty() {
            let avg_time: u64 = times.iter().sum::<u64>() / times.len() as u64;
            self.baseline_time = Some(avg_time);
        }
    }
    
    /// Detect if response size is anomalous
    pub fn is_size_anomaly(&self, size: usize) -> bool {
        if let Some(baseline) = self.baseline_size {
            let deviation = (size as f64 - baseline as f64).abs() / baseline as f64;
            deviation > self.size_threshold
        } else {
            false
        }
    }
    
    /// Detect if response time is anomalous
    pub fn is_time_anomaly(&self, time_ms: u64) -> bool {
        if let Some(baseline) = self.baseline_time {
            let ratio = time_ms as f64 / baseline as f64;
            ratio > self.time_threshold
        } else {
            false
        }
    }
    
    /// Detect anomalies in a response
    pub fn detect(&self, size: usize, time_ms: u64) -> AnomalyReport {
        AnomalyReport {
            size_anomaly: self.is_size_anomaly(size),
            time_anomaly: self.is_time_anomaly(time_ms),
            size_deviation: self.size_deviation(size),
            time_ratio: self.time_ratio(time_ms),
        }
    }
    
    fn size_deviation(&self, size: usize) -> Option<f64> {
        self.baseline_size.map(|baseline| {
            (size as f64 - baseline as f64).abs() / baseline as f64
        })
    }
    
    fn time_ratio(&self, time_ms: u64) -> Option<f64> {
        self.baseline_time.map(|baseline| {
            time_ms as f64 / baseline as f64
        })
    }
}

#[derive(Debug, Clone)]
pub struct AnomalyReport {
    pub size_anomaly: bool,
    pub time_anomaly: bool,
    pub size_deviation: Option<f64>,
    pub time_ratio: Option<f64>,
}

impl AnomalyReport {
    pub fn has_anomaly(&self) -> bool {
        self.size_anomaly || self.time_anomaly
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}
