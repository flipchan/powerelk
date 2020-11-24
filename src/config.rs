use serde_derive::Deserialize;
use std::net::IpAddr;

#[derive(Deserialize, Clone)]
pub struct Conf {
    pub bind: IpAddr,
    pub port: i64,
    pub elasticsearchindex: String,
    pub elasticsearchinstance: String,
    pub cachelocation: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub host: Conf,
}

impl Config {
    pub async fn read_file(filepath: &str) -> Result<Config, toml::de::Error> {
        let toml_str = match tokio::fs::read_to_string(filepath).await {
            Ok(t) => t,
            Err(error) => panic!("Could not open file: {}, error is: {}", filepath, error),
        };
        toml::de::from_str(&toml_str)
    }
}
