use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

pub fn create_new_command(command_name: &str, command_args: Vec<&str>) {

    let mut command = Command::new(command_name)
        .args(command_args)
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