use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub listen: String,
    pub backends: Vec<String>,
}