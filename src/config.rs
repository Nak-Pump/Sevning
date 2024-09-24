use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MainConfig {
    pub application: ApplicationNode,
    pub runtimes: RuntimeNode,
    pub users: UserNode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationNode {
    pub ip: String,
    pub port: String,
    pub verify: bool,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuntimeNode {
    #[serde(flatten)]
    pub command_map: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserNode {
    #[serde(flatten)]
    pub user_map: HashMap<String, String>,
}

pub struct SConfig {
    pub path: String,
    pub config: MainConfig,
}

fn get_default_config() -> MainConfig {
    MainConfig {
        application: ApplicationNode {
            ip: String::from("127.0.0.1"),
            port: String::from("10430"),
            verify: false,
            password: String::from(""),
        },
        runtimes: RuntimeNode {
            command_map: HashMap::new(),
        },
        users: UserNode {
            user_map: HashMap::new(),
        },
    }
}


impl Default for SConfig {
    fn default() -> Self {
        let current_dir = std::env::current_dir();
        if let Ok(current_dir) = current_dir {
            let path = current_dir.to_str().unwrap().to_string();
            SConfig {
                path,
                config: get_default_config(),
            }
        } else {
            SConfig {
                path: "".to_string(),
                config: get_default_config(),
            }
        }
    }
}
impl SConfig {
    pub fn new() -> Self {
        SConfig::default()
    }
    pub fn with_path(path: String) -> Self {
        SConfig {
            path,
            config: get_default_config(),
        }
    }

    pub fn init(self: &mut Self) {
        let path = format!("{}/Sevning.toml", self.path);
        let config_text = std::fs::read_to_string(path);
        match config_text {
            Ok(config_text) => {
                let config = toml::from_str(&config_text);
                if let Ok(config) = config {
                    self.config = config;
                } else {
                    panic!("Failed to parse config.toml");
                }
            }
            Err(_) => {
                panic!("Failed to load config.toml");
            }
        }
    }
}