use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing_subscriber::fmt;
use crate::singleton::COMMAND_MANAGER;

pub struct Commander {
    pub command_owner: String,
    pub command_name: String,
    pub command_args: Vec<String>,
    pub start_time: String,
    pub end_time: String,
    pub uuid: String,
    pub status: i32,
}

impl Default for Commander {
    fn default() -> Self {
        Commander {
            command_owner: String::from(""),
            command_name: String::from(""),
            command_args: Vec::new(),
            start_time: String::from(""),
            end_time: String::from(""),
            uuid: String::from(""),
            status: -1,
        }
    }
}

impl Commander {
    pub fn new(command_owner: String, command_name: String, command_args: Vec<String>) -> Self {
        let uuid = uuid::Uuid::new_v4().to_string();
        let start_time = String::from("");
        let end_time = String::from("");
        Commander {
            command_owner,
            command_name,
            command_args,
            start_time,
            end_time,
            uuid,
            status: -1,
        }
    }

    pub async fn create_new_command(&mut self, tx: mpsc::Sender<String>) {
        let mut command = Command::new(&self.command_name)
            .args(&self.command_args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start command");
        self.start_time = Local::now().to_string();
        println!("Command: {:?} {:?}", self.command_name, self.command_args);

        let stdout = command.stdout.take().expect("Failed to capture stdout");
        let mut reader = BufReader::new(stdout);

        let mut buffer = String::new();

        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    if tx.send(buffer.clone()).await.is_err() {
                        break;
                    }
                    std::io::stdout().flush().unwrap();
                }
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                    break;
                }
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        let status = command.wait().expect("Failed to wait on child");
        self.status = status.code().unwrap_or(-1);
        COMMAND_MANAGER.lock().await.set_statue(self.uuid.clone(), self.status).await;
        self.end_time = Local::now().to_string();
        tx.send("state_exit".to_string()).await.unwrap()
    }
}