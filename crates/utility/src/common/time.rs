use chrono::{Local, TimeZone};

pub fn timestamp_now()->i64 {
    let current_time = Local::now();
    current_time.timestamp()
}
