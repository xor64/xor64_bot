use std::path::Path;
use std::io::Write;
use serde::{Deserialize, Serialize};
use anyhow::{bail, Result};

const DEFAULT_CONFIG: &'static str = include_str!("../config.default.toml");

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    discord: ConfigDiscord,
    database: ConfigDatabase,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDiscord {
    token: String,
    client_id: String,
    client_secret: String,
    app_id: String,
    public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDatabase {
    r#type: ConfigDatabaseType
}


#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigDatabaseType {
    toml,
    json,
    // TODO: Add more database types
}


impl Config {
    pub fn parse(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Self::create_new(path);
        }

        let data;
        match std::fs::read_to_string(path) {
            Ok(d) => {
                data = d;
            }
            Err(e) => {
                log::error!("Unable to open config file: {e}");
                bail!("")
            }
        }

        match toml::from_str::<Self>(&data) {
            Ok(d) => {
                return Ok(d);
            }
            Err(e) => {
                log::error!("Failed to parse config file: {e}");
                bail!("");
            }
        }
    }

    fn create_new(path: &Path) -> Result<Self> {
        let file_r = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path);

        match file_r {
            Ok(mut f) => {
                write!(f, "{}", DEFAULT_CONFIG)?;
            }
            Err(e) => {
                log::error!("Unable to create config file: {e}");
            }
        }

        let cfg= toml::from_str(DEFAULT_CONFIG)
            .expect("Bad default config");
        
        Ok(cfg)
    }
}

