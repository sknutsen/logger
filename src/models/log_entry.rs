use chrono::{DateTime, Utc};

pub struct LogEntry {
    pub time: DateTime<Utc>,
    pub text: String,
    pub source: String,
}

impl LogEntry {
    pub fn new(time: DateTime<Utc>, text: String, source: String) -> LogEntry {
        LogEntry {
            time: time,
            text: text,
            source: source,
        }
    }
}
