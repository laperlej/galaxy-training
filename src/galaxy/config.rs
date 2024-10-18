use anyhow::Result;

const API_KEY_ENV: &str = "GALAXY_ADMIN_API_KEY";
const GALAXY_URL: &str = "https://usegalaxy.ca";

fn get_env_var(name: &str) -> Result<String> {
    match std::env::var(name) {
        Ok(value) => if value.is_empty() {
            Err(anyhow::anyhow!("{} not set", name))
        } else {
            Ok(value)
        },
        Err(e) => Err(anyhow::anyhow!("{} not set: {}", name, e)),
    }
}

fn get_api_key() -> Result<String> {
    get_env_var(API_KEY_ENV)
}

pub struct Config {
    pub galaxy_url: String,
    pub api_key: String,
}

impl Config {
    pub fn new() -> Result<Config> {
        let galaxy_url = GALAXY_URL.to_string();
        let api_key = get_api_key()?;
        Ok(Config {
            galaxy_url,
            api_key,
        })
    }
}
