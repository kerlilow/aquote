use crate::app::App;
use crate::quote::Quote;
use crate::quote_manager::QuoteManager;
use anyhow::Result;
use colored::Colorize;

/// Display recent quotes.
pub fn run(app: &App) -> Result<()> {
    let quotes_path = app.config.data_dir.join("quotes.json");
    let manager = QuoteManager::load(quotes_path, app.config.max_quotes)?;
    let infos: Vec<String> = manager
        .list()
        .iter()
        .rev()
        .map(|q| format_quote(app, q))
        .collect();
    println!("{}", infos.join("\n-----\n"));
    Ok(())
}

/// Format quote.
fn format_quote(app: &App, quote: &Quote) -> String {
    let vendor = app.config.vendors.get(&quote.vendor);
    [
        (
            "Fetched at:",
            Some(
                quote
                    .fetch_time
                    .naive_local()
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ),
        ),
        ("Quote:", Some(quote.quote.to_owned())),
        ("Author:", Some(quote.author.to_owned())),
        ("URL:", quote.url.to_owned()),
        (
            "From:",
            vendor.map(|v| match &v.homepage {
                Some(homepage) => format!("{} ({})", v.name, homepage),
                None => v.name.to_owned(),
            }),
        ),
    ]
    .iter()
    .map(|(heading, value)| match value {
        Some(val) => Some(format!("{} {}", heading.bold(), val)),
        None => None,
    })
    .filter_map(|row| row)
    .collect::<Vec<String>>()
    .join("\n")
}
