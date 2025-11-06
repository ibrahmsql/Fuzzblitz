#![allow(dead_code)]
// Legacy logging module - kept for compatibility
// Most logging is now handled in main.rs with the new output system

use chrono::{prelude::*, Duration};

pub fn format_datetime(dt: DateTime<Local>, long: bool) -> String { 
    if long {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        dt.format("%H:%M:%S").to_string()    
    }
}

pub fn format_duration(duration: Duration) -> String {
    let seconds = duration.num_seconds();
    let ms = duration.num_milliseconds();
    if seconds < 2 {
        format!("{ms}ms")
    } else {
        format!("{seconds}s")
    }
}