use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuoteVendor {
    pub name: String,
    pub homepage: Option<String>,
    pub endpoint: String,
    pub queries: QuoteQueries,
}

#[derive(Debug, Deserialize)]
pub struct QuoteQueries {
    pub quote: String,
    pub author: String,
    pub url: Option<String>,
}
