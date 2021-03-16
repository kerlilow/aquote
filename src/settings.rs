use anyhow::Result;
use config::{Config, File, FileFormat};
use directories::ProjectDirs;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub data_dir: PathBuf,
    pub enable_vendors: Vec<String>,
    pub vendors: HashMap<String, QuoteVendor>,
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
    pub fn new(proj_dirs: &ProjectDirs) -> Result<Self> {
        let mut s = Config::new();
        s.set_default("data_dir", "/var/lib/qotd")?;
        s.merge(
            File::with_name("/etc/qotd/config.toml")
                .format(FileFormat::Toml)
                .required(false),
        )?;
        s.merge(
            File::with_name(proj_dirs.config_dir().join("config").to_str().unwrap())
                .format(FileFormat::Toml)
                .required(false),
        )?;
        let vendors: HashMap<String, QuoteVendor> = s.get("vendors")?;
        s.set_default(
            "enable_vendors",
            vendors.keys().cloned().collect::<Vec<String>>(),
        )?;
        Ok(s.try_into()?)
    }
}
