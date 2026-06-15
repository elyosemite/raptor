use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub listen: String,
    pub backends: Vec<String>,
}