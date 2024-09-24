use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct Commander {
    pub command_owner: String,
    pub command_name: String,
    pub command_args: Vec<String>,
    pub start_time: Duration,
    pub end_time: Duration,
    pub rx: Option<UnboundedReceiverStream<String>>,
    pub uuid: String,
}

impl Default for Commander {
    fn default() -> Self {
        let (_tx, rx) = mpsc::unbounded_channel();
        Commander {
            command_owner: String::from(""),
            command_name: String::from(""),
            command_args: Vec::new(),
            start_time: Duration::new(0, 0),
            end_time: Duration::new(0, 0),
            rx: Some(UnboundedReceiverStream::new(rx)),
            uuid: String::from(""),
        }
    }
}

impl Commander {
    pub fn new(command_owner: String, command_name: String, command_args: Vec<String>) -> Self {
        let (_tx, rx) = mpsc::unbounded_channel();
        let uuid = uuid::Uuid::new_v4().to_string();
        let start_time = Duration::new(0, 0);
        let end_time = Duration::new(0, 0);
        Commander {
            command_owner,
            command_name,
            command_args,
            start_time,
            end_time,
            rx: Some(UnboundedReceiverStream::new(rx)),
            uuid,
        }
    }

    pub fn create_new_command(&self) {
        let mut command = Command::new(&self.command_name)
            .args(&self.command_args)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        let stdout = command.stdout.take().expect("Failed to capture stdout");
        let mut reader = BufReader::new(stdout);

        let mut buffer = String::new();

        loop {
            buffer.clear();
            match reader.read_line(&mut buffer) {
                Ok(0) => break,
                Ok(_) => {
                    print!("{}", buffer);
                    std::io::stdout().flush().unwrap();
                }
                Err(err) => {
                    eprintln!("Error reading line: {}", err);
                    break;
                }
            }
            sleep(Duration::from_millis(100));
        }

        let status = command.wait().expect("Failed to wait on child");
        println!("Command exited with status: {}", status);
    }
}