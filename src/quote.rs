use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Quote {
    pub quote: String,
    pub author: String,
    pub url: Option<String>,
    pub vendor: String,
    pub fetch_time: DateTime<Utc>,
}
