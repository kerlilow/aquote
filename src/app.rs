use crate::settings::Settings;
use anyhow::{Context, Result};

pub struct App {
    pub config: Settings,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: Settings::new().context("Failed to initialize configuration")?,
        })
    }
}
