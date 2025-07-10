use std::collections::HashMap;
use std::env;
use std::fs::{create_dir_all, read_to_string};
use std::path::PathBuf;

/// Structure representing the key configuration options (such as API key)
#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: Option<String>,
    pub model: String,
}

impl Config {
    /// Loads the configuration from $HOME/.config/cliquery/.cliqueryrc
    pub fn load() -> Self {
        let path = get_config_file();
        let mut config = HashMap::new();

        if let Ok(content) = read_to_string(&path) {
            for line in content.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    config.insert(key.trim().to_string(), value.trim().to_string());
                }
            }
        }

        let api_key = config
            .get("OPENAI_API_KEY")
            .cloned()
            .or_else(|| env::var("OPENAI_API_KEY").ok());

        let model = config
            .get("DEFAULT_MODEL")
            .cloned()
            .unwrap_or_else(|| "gpt-4o".to_string());

        Config { api_key, model }
    }
}

fn get_config_file() -> PathBuf {
    let mut home = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"));
    home.push(".config/cliquery/.cliqueryrc");

    if let Some(parent) = home.parent() {
        let _ = create_dir_all(parent);
    }

    home
}
