mod model;

pub use model::Config;

use std::fs;

pub fn load() -> anyhow::Result<Config> {
    let content =
        fs::read_to_string("raptor.toml")?;

    let config =
        toml::from_str(&content)?;

    Ok(config)
}

#[cfg(test)]
mod tests;