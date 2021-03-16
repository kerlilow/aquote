use crate::quote::Quote;
use anyhow::{ensure, Result};
use std::collections::VecDeque;
use std::path::PathBuf;

pub struct QuoteManager {
    /// Quotes
    quotes: VecDeque<Quote>,
    /// Path to quotes file.
    path: PathBuf,
    /// Maximum number of quotes to store.
    max_quotes: usize,
}

impl QuoteManager {
    /// Create new QuoteManager instance.
    ///
    /// This constructor does not load quotes from the specified file path, but uses the provided
    /// quotes deque. To load from a quotes file, use `QuoteManager::load` instead.
    ///
    /// # Errors
    /// An error is returned if `max_quotes` is less than 1.
    pub fn new(
        quotes: impl Into<VecDeque<Quote>>,
        path: impl Into<PathBuf>,
        max_quotes: usize,
    ) -> Result<Self> {
        ensure!(
            max_quotes > 0,
            "Maximum number of quotes must be greater than 0"
        );
        let mut quotes = quotes.into();
        quotes.truncate(max_quotes);
        Ok(Self {
            quotes: quotes.into(),
            path: path.into(),
            max_quotes: max_quotes,
        })
    }

    /// Load quotes from file.
    ///
    /// If the quotes file does not exist, initialize the QuoteManager with an empty quote deque.
    ///
    /// # Errors
    /// An error is returned if `max_quotes` is less than 1.
    pub fn load(path: impl Into<PathBuf>, max_quotes: usize) -> Result<Self> {
        let path: PathBuf = path.into();
        let quotes = match std::fs::read_to_string(&path) {
            Ok(data) => VecDeque::from(
                serde_json::from_str::<Vec<Quote>>(&data)?
                    .into_iter()
                    .rev()
                    .take(max_quotes)
                    .collect::<Vec<Quote>>(),
            ),
            Err(_) => VecDeque::new(),
        };
        Self::new(quotes, path, max_quotes)
    }

    /// Save quotes to file.
    pub fn save(&self) -> Result<()> {
        if let Some(parent_dir) = self.path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }
        let quotes: Vec<&Quote> = self.quotes.iter().rev().collect();
        std::fs::write(&self.path, serde_json::to_string_pretty(&quotes)?)?;
        Ok(())
    }

    /// Push quote, truncating the quote deque if it exceeds the maximum number of quotes.
    pub fn push(&mut self, quote: Quote) {
        self.quotes.truncate(self.max_quotes - 1);
        self.quotes.push_front(quote);
    }

    /// List quotes, ordered from most recent to least recent.
    pub fn list(&self) -> &VecDeque<Quote> {
        &self.quotes
    }

    /// Get latest quote.
    pub fn get(&self) -> Option<&Quote> {
        self.quotes.front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    fn make_quote(quote: &str) -> Quote {
        Quote {
            quote: quote.to_owned(),
            author: "Test Author".to_owned(),
            url: None,
            vendor: "testvendor".to_owned(),
            fetch_time: DateTime::parse_from_rfc3339("2021-01-01T10:00:00Z")
                .unwrap()
                .into(),
        }
    }

    #[test]
    fn new() {
        assert!(QuoteManager::new(vec![make_quote("Quote 1")], "/dev/null", 3,).is_ok());
    }

    #[test]
    fn new_max_queues_too_small() {
        assert!(QuoteManager::new(vec![make_quote("Quote 1")], "/dev/null", 0,).is_err());
    }

    #[test]
    fn list() {
        let quotes = vec![make_quote("Quote 1"), make_quote("Quote 2")];
        let manager = QuoteManager::new(quotes.clone(), "/dev/null", 3).unwrap();
        assert!(manager.list().iter().eq(quotes.iter()));
    }

    #[test]
    fn new_truncates() {
        let quotes = vec![make_quote("Quote 1"), make_quote("Quote 2")];
        let max_quotes = 1;
        // `max_quotes` must be less than number of quotes for the test to be meaningful.
        assert!(max_quotes < quotes.len());
        let manager = QuoteManager::new(quotes.clone(), "/dev/null", max_quotes).unwrap();
        assert!(manager.list().iter().eq(quotes.iter().take(max_quotes)));
    }

    #[test]
    fn get() {
        let quotes = vec![make_quote("Quote 1"), make_quote("Quote 2")];
        let manager = QuoteManager::new(quotes.clone(), "/dev/null", 3).unwrap();
        assert_eq!(manager.get().unwrap().to_owned(), quotes[0]);
    }

    #[test]
    fn push() {
        let mut manager = QuoteManager::new(vec![], "/dev/null", 2).unwrap();
        manager.push(make_quote("Quote 1"));
        assert!(manager.list().iter().eq(vec![make_quote("Quote 1")].iter()));
        manager.push(make_quote("Quote 2"));
        assert!(manager
            .list()
            .iter()
            .eq(vec![make_quote("Quote 2"), make_quote("Quote 1")].iter()));
        manager.push(make_quote("Quote 3"));
        assert!(manager
            .list()
            .iter()
            .eq(vec![make_quote("Quote 3"), make_quote("Quote 2")].iter()));
    }
}
