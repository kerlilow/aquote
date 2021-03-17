use crate::app::App;
use crate::quote::Quote;
use crate::quote_manager::QuoteManager;
use anyhow::{anyhow, ensure, Result};
use rand::prelude::*;
use retry::{delay::Fixed, retry};

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
    Quote::fetch(vendor_key, vendor)
}
