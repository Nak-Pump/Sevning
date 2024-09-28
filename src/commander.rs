use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing_subscriber::fmt;

pub struct Commander {
    pub command_owner: String,
    pub command_name: String,
    pub command_args: Vec<String>,
    pub start_time: Duration,
    pub end_time: Duration,
    pub uuid: String,
    pub latest_text: String,
    pub status: i32,
}

impl Default for Commander {
    fn default() -> Self {
        Commander {
            command_owner: String::from(""),
            command_name: String::from(""),
            command_args: Vec::new(),
            start_time: Duration::new(0, 0),
            end_time: Duration::new(0, 0),
            uuid: String::from(""),
            latest_text: String::from(""),
            status: -1,
        }
    }
}

impl Commander {
    pub fn new(command_owner: String, command_name: String, command_args: Vec<String>) -> Self {
        let uuid = uuid::Uuid::new_v4().to_string();
        let start_time = Duration::new(0, 0);
        let end_time = Duration::new(0, 0);
        let latest_text = String::from("");
        Commander {
            command_owner,
            command_name,
            command_args,
            start_time,
            end_time,
            uuid,
            latest_text,
            status: -1,
        }
    }

    pub async fn create_new_command(&mut self,tx: mpsc::Sender<String>) {
        let mut command = Command::new(&self.command_name)
            .args(&self.command_args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start command");

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
        tx.send("state_exit".to_string()).await.unwrap()
    }

    pub fn get_command_output(&self) -> String {
        self.latest_text.clone()
    }
}