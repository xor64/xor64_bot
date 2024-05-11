use std::path::Path;

mod logger;
mod config;
mod client;
fn main() -> anyhow::Result<()> {
    logger::init();
    let cfg = config::Config::parse(Path::new("config.toml"))?;
    
    Ok(())
}
