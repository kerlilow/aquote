use config::{Config, ConfigError, File, FileFormat};
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub quote_vendors: HashMap<String, QuoteVendor>,
}

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

impl Settings {
    pub fn new(proj_dirs: &ProjectDirs) -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default").format(FileFormat::Toml))?;
        s.merge(
            File::with_name(proj_dirs.config_dir().join("config").to_str().unwrap())
                .format(FileFormat::Toml)
                .required(false),
        )?;
        s.try_into()
    }
}
