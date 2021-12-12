use chrono::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn format_timestamp(timestamp: u64, format_str: &'static str) -> String {
    let date = UNIX_EPOCH + Duration::from_millis(timestamp);
    let datetime = DateTime::<Utc>::from(date);

    datetime.format(format_str).to_string()
}
