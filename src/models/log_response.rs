//use chrono::{DateTime, Utc};

#[derive(FromForm)]
pub struct LogResponse {
    //pub time: DateTime<Utc>,
    pub text: String,
    pub source: String,
}
