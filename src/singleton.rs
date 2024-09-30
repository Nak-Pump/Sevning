use std::collections::HashMap;
use std::sync::{Arc};
use once_cell::sync::Lazy;
use tokio::sync::{Mutex, RwLock};
use crate::commander::Commander;

pub struct CommandManager {
    command_map: HashMap<String, Arc<RwLock<Commander>>>,
}

impl CommandManager {
    pub fn new() -> Self {
        CommandManager {
            command_map: HashMap::new(),
        }
    }

    pub async fn add_command(&mut self, command: Arc<RwLock<Commander>>) {
        self.command_map.insert(command.read().await.uuid.clone(), command.clone());
    }

    pub fn remove_command(&mut self, uuid: String) {
        self.command_map.remove(&uuid);
    }

    pub fn get_command(&self, uuid: String) -> Option<&Arc<RwLock<Commander>>> {
        self.command_map.get(&uuid)
    }
}

pub static COMMAND_MANAGER: Lazy<Mutex<CommandManager>> = Lazy::new(|| {
    Mutex::new(CommandManager::new())
});