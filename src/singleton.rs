use std::collections::HashMap;
use std::sync::{Arc};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use crate::commander::Commander;

struct CommandManager {
    command_map: HashMap<String, Arc<Mutex<Commander>>>,
}

impl CommandManager {
    fn new() -> Self {
        CommandManager {
            command_map: HashMap::new(),
        }
    }

    async fn add_command(&mut self, command: Arc<Mutex<Commander>>) {
        self.command_map.insert(command.lock().await.uuid.clone(), command);
    }

    fn remove_command(&mut self, uuid: String) {
        self.command_map.remove(&uuid);
    }

    fn get_command(&self, uuid: String) -> Option<&Arc<Mutex<Commander>>> {
        self.command_map.get(&uuid)
    }
}

pub static COMMAND_MANAGER: Lazy<Mutex<CommandManager>> = Lazy::new(|| {
    Mutex::new(CommandManager::new())
});