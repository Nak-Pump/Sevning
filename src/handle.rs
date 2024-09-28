use std::string::String;
use std::convert::Infallible;
use std::time::Duration;
use futures_util::StreamExt;
use salvo::prelude::*;
use salvo::sse::{self, SseEvent};
use tokio::sync::mpsc;
use tokio::time::interval;
use tokio_stream::wrappers::{IntervalStream, UnboundedReceiverStream};
use crate::config;
/***
    * This is a simple handler that returns a string "Hello World"
 */
#[handler]
pub async fn hello_handler() -> &'static str {
    "Hello World"
}


fn sse_text(text: &str) -> Result<SseEvent, Infallible> {
    Ok(SseEvent::default().text(text))
}

async fn send_sse_message_demo(tx: mpsc::UnboundedSender<&str>) {
    let mut interval = IntervalStream::new(interval(Duration::from_secs(1)));
    let messages = vec![
        "Welcome to the chat!",
        "This is a simulated stream response.",
        "ChatGPT is sending messages asynchronously.",
        "Goodbye!",
    ];
    for msg in messages {
        interval.next().await;
        tx.send(msg).unwrap();
    }
}

fn parse_command_args(command_args: String) -> Vec<String> {
    let mut command_args_vec: Vec<String> = Vec::new();
    let mut command_args_split = command_args.split(" ");
    for arg in command_args_split {
        command_args_vec.push(arg.to_string());
    }
    command_args_vec
}


fn generate_true_command(command_name: String, command_args: Vec<String>) -> String {
    let mut config = config::SConfig::new();
    config.init();
    let command_map = &config.config.runtimes.command_map;
    let mut command_line = command_map.get(&command_name).unwrap().to_string();

    let mut left_position: Vec<usize> = Vec::new();
    let mut right_position: Vec<usize> = Vec::new();

    let chars: Vec<(usize, char)> = command_line.char_indices().collect();

    for (i, (_, char)) in chars.iter().enumerate() {
        if *char == '{' {
            left_position.push(i);
        }
        if *char == '}' {
            right_position.push(i);
        }
    }

    if left_position.len() != right_position.len() || left_position.len() != command_args.len() {
        panic!("Command line format error");
    }

    for i in (0..left_position.len()).rev() {
        let left = left_position[i];
        let right = right_position[i];
        let replace_str: String = chars[left..=right].iter().map(|(_, c)| c).collect();
        command_line = command_line.replacen(&replace_str, &command_args[i], 1);
    }
    command_line
}


#[handler]
pub async fn sevning_handler(req: &mut Request, res: &mut Response) {
    let mut token = req.query("token").unwrap_or("default").to_string();
    let mut command_name = req.query("command_name").unwrap_or("default").to_string();
    let mut command_args = req.query("command_args").unwrap_or("default").to_string();

    let command_args_vec = parse_command_args(command_args);
    let true_command = generate_true_command(command_name, command_args_vec);

    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::spawn(send_sse_message_demo(tx));

    let stream = rx.map(|msg| sse_text(msg));
    SseKeepAlive::new(stream).stream(res);
}

#[cfg(test)]
mod test {
    #[test]
    fn generate_true_command() {
        let command_name = "echo".to_string();
        let command_args = vec!["Hello".to_string()];
        let true_command = super::generate_true_command(command_name, command_args);
        assert_eq!(true_command, "echo Hello");
    }
}
