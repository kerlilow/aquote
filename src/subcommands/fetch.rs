use crate::app::App;
use crate::quote::Quote;
use crate::quote_manager::QuoteManager;
use anyhow::{anyhow, ensure, Context, Result};
use rand::prelude::*;
use retry::{delay::Fixed, retry};
use serde::de::DeserializeOwned;

pub fn run(app: &App) -> Result<()> {
    ensure!(
        !app.config.vendors.is_empty(),
        "No quote vendors configured"
    );
    let mut rng = rand::thread_rng();
    let quote = retry(Fixed::from_millis(3000).take(2), || {
        let vendor_key = app.config.enable_vendors.choose(&mut rng).unwrap();
        fetch_quote(app, vendor_key)
    })
    .map_err(|e| match e {
        retry::Error::Operation { error, tries, .. } => {
            error.context(format!("Failed fetching quote after {} attempts", tries))
        }
        retry::Error::Internal(msg) => anyhow!("An internal error occurred: {}", msg),
    })?;
    let quotes_path = app.config.data_dir.join("quotes.json");
    let mut manager = QuoteManager::load(quotes_path, app.config.max_quotes)?;
    manager.push(quote);
    manager.save()?;
    Ok(())
}

/// Fetch quote from quote vendor.
fn fetch_quote(app: &App, vendor_key: &str) -> Result<Quote> {
    let vendor = &app
        .config
        .vendors
        .get(vendor_key)
        .ok_or(anyhow!("Vendor \"{}\" not found", vendor_key))?;
    let res = reqwest::blocking::get(&vendor.endpoint)
        .context("Failed to fetch quote")?
        .text()?;
    Ok(Quote {
        quote: query_json(&vendor.queries.quote, &res).context("Failed to parse quote")?,
        author: query_json(&vendor.queries.author, &res).context("Failed to parse author")?,
        url: if let Some(q) = &vendor.queries.url {
            query_json(&q, &res).context("Failed to parse URL")?
        } else {
            None
        },
        vendor: vendor_key.to_owned(),
        fetch_time: chrono::Utc::now(),
    })
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
