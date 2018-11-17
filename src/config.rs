use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use url::Url;

/// Struct for storage params from config.toml
#[derive(Deserialize)]
pub struct Config {
    //wow_wiki_sites: Vec<String>,
    telegram_token: String,
    telegram_endpoint: String,
    pub timeout: u64
}

impl Config {
    /// Constructor. path - path to config file with json settings
    pub fn new(path: &str) -> Config {
        let file_name = Path::new(path);

        let mut file = File::open(&file_name).unwrap();

        let mut content = String::new();

        // Read content of file to string
        file.read_to_string(&mut content).unwrap();

        // Convert string with config settings to struct
        serde_json::from_str(&content.to_string()).unwrap()
    }

    /// Get url endpoint for send request to telegram bot
    pub fn get_telegram_bot_endpoint(&self) -> Url {
        let endpoint = self.telegram_endpoint.to_owned();
        let token = self.telegram_token.to_owned();
        Url::parse(&format!("{}/bot{}/dummy", endpoint, token)).unwrap()
    }
}
