use anyhow::{anyhow, ensure, Context, Result};
use clap::{AppSettings, Clap};
use colored::Colorize;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use quote::Quote;
use quote_manager::QuoteManager;
use rand::prelude::*;
use retry::{delay::Fixed, retry};
use serde::de::DeserializeOwned;

mod quote;
mod quote_manager;
mod settings;

const MAX_QUOTES: usize = 5;

lazy_static! {
    static ref PROJ_DIRS: ProjectDirs = ProjectDirs::from("me", "kerlilow", "qotd")
        .expect("Failed to initialize project directories");
    static ref CONFIG: settings::Settings =
        settings::Settings::new(&PROJ_DIRS).expect("Failed to load configuration file");
}

#[derive(Clap)]
#[clap(
    version = "0.1",
    author = "Kerli Low <kerlilow@gmail.com>",
    setting = AppSettings::ColoredHelp,
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Clap)]
enum Subcommand {
    Show,
    Fetch,
    Recent,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        Subcommand::Show => show(),
        Subcommand::Fetch => fetch(),
        Subcommand::Recent => recent(),
    }
}

/// Show latest quote.
fn show() -> Result<()> {
    let quotes_path = PROJ_DIRS.data_dir().join("quotes.json");
    let manager = QuoteManager::load(quotes_path, MAX_QUOTES)?;
    match manager.get() {
        Some(q) => println!("{}\n—{}", q.quote.italic(), q.author),
        None => println!("No quotes available, run `qotd fetch` to fetch new quote"),
    }
    Ok(())
}

/// Fetch new quote from a random quote vendor.
fn fetch() -> Result<()> {
    ensure!(
        !CONFIG.quote_vendors.is_empty(),
        "No quote vendors configured"
    );
    let mut rng = rand::thread_rng();
    let quote = retry(Fixed::from_millis(3000).take(2), || {
        let vendor_key = CONFIG.quote_vendors.keys().choose(&mut rng).unwrap();
        fetch_quote(vendor_key)
    })
    .map_err(|e| match e {
        retry::Error::Operation { error, tries, .. } => {
            error.context(format!("Failed fetching quote after {} attempts", tries))
        }
        retry::Error::Internal(msg) => anyhow!("An internal error occurred: {}", msg),
    })?;
    let quotes_path = PROJ_DIRS.data_dir().join("quotes.json");
    let mut manager = QuoteManager::load(quotes_path, MAX_QUOTES)?;
    manager.push(quote);
    manager.save()?;
    Ok(())
}

/// Fetch quote from quote vendor.
fn fetch_quote(vendor_key: &str) -> Result<Quote> {
    let vendor = &CONFIG.quote_vendors[vendor_key];
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
    let val = jq_rs::run(query, json_str).map_err(|e| anyhow!(e.to_string()))?;
    Ok(serde_json::from_str(&val)?)
}

/// Display recent quotes.
fn recent() -> Result<()> {
    let quotes_path = PROJ_DIRS.data_dir().join("quotes.json");
    let manager = QuoteManager::load(quotes_path, MAX_QUOTES)?;
    let infos: Vec<String> = manager.list().iter().rev().map(format_quote).collect();
    println!("{}", infos.join("\n-----\n"));
    Ok(())
}

/// Format quote.
fn format_quote(quote: &Quote) -> String {
    let vendor = CONFIG.quote_vendors.get(&quote.vendor);
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
