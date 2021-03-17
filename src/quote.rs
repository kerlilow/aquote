use crate::quote_vendor::QuoteVendor;
use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Quote {
    pub quote: String,
    pub author: String,
    pub url: Option<String>,
    pub vendor: String,
    pub fetch_time: DateTime<Utc>,
}

impl Quote {
    pub fn fetch(vendor_key: &str, vendor: &QuoteVendor) -> Result<Self> {
        let res = reqwest::blocking::get(&vendor.endpoint)
            .context("Failed to fetch quote")?
            .text()?;
        Self::from_json(vendor_key, vendor, &res, chrono::Utc::now())
    }

    pub fn from_json(
        vendor_key: &str,
        vendor: &QuoteVendor,
        json_str: &str,
        fetch_time: DateTime<Utc>,
    ) -> Result<Self> {
        Ok(Self {
            quote: query_json(&vendor.queries.quote, json_str).context("Failed to parse quote")?,
            author: query_json(&vendor.queries.author, json_str)
                .context("Failed to parse author")?,
            url: if let Some(q) = &vendor.queries.url {
                query_json(&q, json_str).context("Failed to parse URL")?
            } else {
                None
            },
            vendor: vendor_key.to_owned(),
            fetch_time,
        })
    }
}

/// Query and deserialize value from JSON string.
fn query_json<T>(query: &str, json_str: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let val = ajson::get(json_str, query).ok_or(anyhow!("Failed to query value"))?;
    let val_str = if val.is_string() {
        // `ajson::Value::as_str` returns the contained string directly, so it is necessary to wrap
        // it in quotes to make it parseable by serde_json.
        format!("\"{}\"", val.as_str())
    } else {
        val.as_str().to_owned()
    };
    Ok(serde_json::from_str(&val_str)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quote_vendor::{QuoteQueries, QuoteVendor};

    fn make_vendor(queries: QuoteQueries) -> QuoteVendor {
        QuoteVendor {
            name: "Test".to_owned(),
            homepage: None,
            endpoint: "https://test".to_owned(),
            queries,
        }
    }

    #[test]
    fn query_json_str() {
        assert_eq!(
            query_json::<String>("s", r#"{ "s": "test" }"#).unwrap(),
            "test"
        );
    }

    #[test]
    fn from_json() {
        let vendor = make_vendor(QuoteQueries {
            quote: "quote".to_owned(),
            author: "author".to_owned(),
            url: None,
        });
        let fetch_time = chrono::Utc::now();
        assert_eq!(
            Quote::from_json(
                "test",
                &vendor,
                r#"{
                  "quote": "Test quote.",
                  "author": "Test Author",
                }"#,
                fetch_time,
            )
            .unwrap(),
            Quote {
                quote: "Test quote.".to_owned(),
                author: "Test Author".to_owned(),
                url: None,
                vendor: "test".to_owned(),
                fetch_time
            }
        )
    }

    #[test]
    fn from_json_url() {
        let vendor = make_vendor(QuoteQueries {
            quote: "quote".to_owned(),
            author: "author".to_owned(),
            url: Some("url".to_owned()),
        });
        let fetch_time = chrono::Utc::now();
        assert_eq!(
            Quote::from_json(
                "test",
                &vendor,
                r#"{
                  "quote": "Test quote.",
                  "author": "Test Author",
                  "url": "https://test/quote",
                }"#,
                fetch_time,
            )
            .unwrap(),
            Quote {
                quote: "Test quote.".to_owned(),
                author: "Test Author".to_owned(),
                url: Some("https://test/quote".to_owned()),
                vendor: "test".to_owned(),
                fetch_time
            }
        )
    }
}
