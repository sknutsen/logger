use chrono::prelude::*;

use crate::models::log_entry::LogEntry;

pub fn create_log(text: String, source: String) -> LogEntry {
    let time: DateTime<Utc> = Utc::now();
    let entry = LogEntry::new(time, text, source);

    entry
}
