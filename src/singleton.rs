use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::commander::Commander;

struct CommandManager {
    command_map: HashMap<String, Commander>,
}

impl CommandManager {
    fn new() -> Self {
        CommandManager {
            command_map: HashMap::new(),
        }
    }

    fn add_command(&mut self, command: Commander) {
        self.command_map.insert(command.uuid.clone(), command);
    }

    fn remove_command(&mut self, uuid: String) {
        self.command_map.remove(&uuid);
    }

    fn get_command(&self, uuid: String) -> Option<&Commander> {
        self.command_map.get(&uuid)
    }
}

pub static COMMAND_MANAGER: Lazy<Mutex<CommandManager>> = Lazy::new(|| {
    Mutex::new(CommandManager::new())
});