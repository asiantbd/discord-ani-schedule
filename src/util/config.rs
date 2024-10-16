use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize)]
pub struct Config {
    pub discord_token: String,
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let file = File::open("config.json")?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}
