use crate::app::App;
use crate::quote::Quote;
use crate::quote_manager::QuoteManager;
use anyhow::Result;
use clap::{AppSettings, Clap};
use colored::Colorize;
use std::str::FromStr;

#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Show specific attribute [quote, author].
    attribute: Option<ShowAttr>,
}

enum ShowAttr {
    Quote,
    Author,
}

impl FromStr for ShowAttr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "quote" => Ok(ShowAttr::Quote),
            "author" => Ok(ShowAttr::Author),
            _ => Err("invalid attribute"),
        }
    }
}

/// Show latest quote.
pub fn run(app: &App, opts: &Opts) -> Result<()> {
    let quotes_path = app.config.data_dir.join("quotes.json");
    let manager = QuoteManager::load(quotes_path, app.config.max_quotes)?;
    match manager.get() {
        Some(q) => show_quote(opts, q),
        None => println!("No quotes available, run `aquote fetch` to fetch a quote"),
    }
    Ok(())
}

fn show_quote(opts: &Opts, quote: &Quote) {
    match opts.attribute {
        Some(ShowAttr::Quote) => println!("{}", quote.quote),
        Some(ShowAttr::Author) => println!("{}", quote.author),
        None => println!("{}\n—{}", quote.quote.italic(), quote.author),
    }
}
