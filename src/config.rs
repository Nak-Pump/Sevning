use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MainConfig {}

#[derive(Debug, Deserialize, Serialize)]
struct ApplicationNode {
    ip: String,
    port: String,
    verify: bool,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RuntimeNode {
    command_map: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserNode {
    user_map: HashMap<String, String>,
}

pub struct SConfig {
    pub path: String,
}

impl Default for SConfig {
    fn default() -> Self {
        let current_dir = std::env::current_dir();
        if let Ok(current_dir) = current_dir {
            let path = current_dir.to_str().unwrap().to_string();
            SConfig { path }
        } else {
            SConfig { path: ".".to_string() }
        }
    }
}
impl SConfig {
    pub fn new() -> Self {
        SConfig::default()
    }
    pub fn with_path(path: String) -> Self {
        SConfig { path }
    }
}