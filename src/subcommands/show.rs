use crate::app::App;
use crate::quote_manager::QuoteManager;
use anyhow::Result;
use colored::Colorize;

/// Show latest quote.
pub fn run(app: &App) -> Result<()> {
    let quotes_path = app.config.data_dir.join("quotes.json");
    let manager = QuoteManager::load(quotes_path, app.config.max_quotes)?;
    match manager.get() {
        Some(q) => println!("{}\n—{}", q.quote.italic(), q.author),
        None => println!("No quotes available, run `qotd fetch` to fetch new quote"),
    }
    Ok(())
}
