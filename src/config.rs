use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use crate::obsidian_handler::ObsidianConfig;
use crate::sources::github::GitHubSource;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config{
    pub github_config: GitHubSource,
    pub poll_interval: u64, // in minutes
    pub obsidian_handler: ObsidianConfig,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let file = std::fs::File::open(path).expect("Could not open file");
        serde_yaml::from_reader(file).expect("Could not parse file")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::from_file("config.yaml");
        assert!(config.github_config.repos.len() > 0);
    }
}
