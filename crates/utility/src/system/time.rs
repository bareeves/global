use chrono::{Local};
use chrono::{TimeZone, Utc};

pub fn format_timestamp_to_gmt_string(timestamp: i64) -> String {
    let dt = Utc.timestamp(timestamp, 0);
    dt.to_rfc2822()
}
pub fn timestamp_now()->i64 {
    let current_time = Local::now();
    current_time.timestamp()
}
