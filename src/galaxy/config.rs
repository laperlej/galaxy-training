use anyhow::Result;

pub const API_KEY_ENV: &str = "GALAXY_ADMIN_API_KEY";
pub const GALAXY_HOSTNAME_ENV: &str = "GALAXY_HOSTNAME";

pub trait EnvVarProvider {
    fn get(&self, key: &str) -> std::result::Result<String, std::env::VarError>;
    fn set(&mut self, key: &str, value: &str);
    fn remove(&mut self, key: &str);
}

pub struct RealEnvVarProvider;

impl RealEnvVarProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl EnvVarProvider for RealEnvVarProvider {
    fn get(&self, key: &str) -> std::result::Result<String, std::env::VarError> {
        std::env::var(key)
    }

    fn set(&mut self, key: &str, value: &str) {
        std::env::set_var(key, value)
    }

    fn remove(&mut self, key: &str) {
        std::env::remove_var(key)
    }
}

fn get_env_var(name: &str, provider: &dyn EnvVarProvider) -> Result<String> {
    match provider.get(name) {
        Ok(value) => if value.is_empty() {
            Err(anyhow::anyhow!("{} not set", name))
        } else {
            Ok(value)
        },
        Err(e) => Err(anyhow::anyhow!("{} not set: {}", name, e)),
    }
}

fn get_api_key(provider: &dyn EnvVarProvider) -> Result<String> {
    get_env_var(API_KEY_ENV, provider)
}

pub fn get_galaxy_url(provider: &dyn EnvVarProvider) -> Result<String> {
    let hostname = get_env_var(GALAXY_HOSTNAME_ENV, provider)?;
    Ok(format!("https://{}", hostname))
}

pub struct Config {
    pub galaxy_url: String,
    pub api_key: String,
}

impl Config {
    pub fn new(provider: &dyn EnvVarProvider) -> Result<Config> {
        let galaxy_url = get_galaxy_url(provider)?;
        let api_key = get_api_key(provider)?;
        Ok(Config {
            galaxy_url,
            api_key,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockEnvVarProvider {
        env_vars: std::collections::HashMap<String, String>,
    }

    impl MockEnvVarProvider {
        pub fn new() -> Self {
            Self {
                env_vars: std::collections::HashMap::new(),
            }
        }
    }

    impl EnvVarProvider for MockEnvVarProvider {
        fn get(&self, key: &str) -> std::result::Result<String, std::env::VarError> {
            self.env_vars.get(key).map(|v| v.to_string()).ok_or(std::env::VarError::NotPresent)
        }

        fn set(&mut self, key: &str, value: &str) {
            self.env_vars.insert(key.to_string(), value.to_string());
        }

        fn remove(&mut self, key: &str) {
            self.env_vars.remove(key);
        }
    }

    #[test]
    fn test_get_env_var() {
        let mut provider = RealEnvVarProvider::new();
        let test_key = "test";
        provider.set(test_key, "test");
        assert_eq!(get_env_var("test", &provider).unwrap(), "test");
        provider.set(test_key, "");
        assert!(get_env_var("test", &provider).is_err());
        provider.remove(test_key);
        assert!(get_env_var("test", &provider).is_err());

    }

    #[test]
    fn test_get_api_key() {
        let mut mock_provider = MockEnvVarProvider::new();
        mock_provider.set(API_KEY_ENV, "test");
        assert_eq!(get_api_key(&mock_provider).unwrap(), "test");
    }

    #[test]
    fn test_get_api_key_not_set() {
        let mut mock_provider = MockEnvVarProvider::new();
        mock_provider.remove(API_KEY_ENV);
        assert!(get_api_key(&mock_provider).is_err());
    }

    #[test]
    fn test_get_galaxy_url() {
        let mut mock_provider = MockEnvVarProvider::new();
        mock_provider.set(GALAXY_HOSTNAME_ENV, "test.ca");
        assert_eq!(get_galaxy_url(&mock_provider).unwrap(), "https://test.ca");
    }

    #[test]
    fn test_get_galaxy_url_not_set() {
        let mut mock_provider = MockEnvVarProvider::new();
        mock_provider.remove(GALAXY_HOSTNAME_ENV);
        assert!(get_galaxy_url(&mock_provider).is_err());
    }

    #[test]
    fn test_config_new() {
        let mut mock_provider = MockEnvVarProvider::new();
        mock_provider.set(API_KEY_ENV, "test");
        mock_provider.set(GALAXY_HOSTNAME_ENV, "test.ca");
        let config = Config::new(&mock_provider);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.galaxy_url, "https://test.ca");
        assert_eq!(config.api_key, "test");
    }

    #[test]
    fn test_config_new_not_set() {
        let mut mock_provider = MockEnvVarProvider::new();
        mock_provider.remove(API_KEY_ENV);
        let config = Config::new(&mock_provider);
        assert!(config.is_err());
    }
}
