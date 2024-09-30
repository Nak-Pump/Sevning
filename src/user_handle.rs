use std::string::String;
use std::convert::Infallible;
use std::sync::{Arc};
use tokio::sync::{Mutex, RwLock}; // Use Tokio's Mutex
use std::time::Duration;
use futures_util::StreamExt;
use salvo::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use salvo::http::HeaderValue;
use salvo::prelude::*;
use salvo::sse::{self, SseEvent};
use tokio::sync::mpsc;
use tokio::time::interval;
use tokio_stream::wrappers::{IntervalStream, UnboundedReceiverStream};
use crate::commander::Commander;
use crate::config;
use crate::singleton::COMMAND_MANAGER;
/***
    * This is a simple handler that returns a string "Hello World"
 */
#[handler]
pub async fn hello_handler() -> &'static str {
    "Hello World"
}


fn sse_text(text: String) -> Result<SseEvent, Infallible> {
    Ok(SseEvent::default().text(text))
}


async fn sse_message(tx: mpsc::UnboundedSender<String>, commander: Arc<RwLock<Commander>>) {
    let (output_tx, mut output_rx) = mpsc::channel(1000);
    // Create a new command
    let commander_clone = commander.clone();
    tokio::spawn(async move {
        let mut cmd = commander_clone.write().await;
        cmd.create_new_command(output_tx).await
    });
    // read output and send to client
    let mut send_interval = IntervalStream::new(interval(Duration::from_millis(1000)));
    while let Some(_) = send_interval.next().await {
        if let Some(message) = output_rx.recv().await {
            // if message is state_exit, break the loop
            if message == "state_exit" { break; }
            let result = tx.send(message);
            if result.is_err() {
                break;
            }
        }
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

// This handler is used to execute a command and send the output to the client
#[handler]
pub async fn sevning_handler(req: &mut Request, res: &mut Response) {
    // Set Headers
    res.headers_mut()
        .insert(CONTENT_TYPE, HeaderValue::from_static("text/event-stream; charset=utf-8"));
    res.headers_mut()
        .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    // get params
    let token = req.query("token").unwrap_or("default").to_string();
    let command_name = req.query("command_name").unwrap_or("default").to_string();
    let command_args = req.query("command_args").unwrap_or("default").to_string();

    if token == "default" || command_name == "default" || command_args == "default" {
        res.write_body("Failed To Parse Param").expect("Failed to write response");
        return;
    }

    // parse command name and command args
    let command_args_vec = parse_command_args(command_args);
    let true_command = generate_true_command(command_name.clone(), command_args_vec.clone());
    let args_command = true_command.replace(format!("{} ", command_name).as_str(), "");
    let command_args_vec = parse_command_args(args_command);

    // generate channel to send message
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    let rx = UnboundedReceiverStream::new(rx);
    let commander = Arc::new(RwLock::new(Commander::new(token.clone(), command_name, command_args_vec)));

    COMMAND_MANAGER.lock().await.add_command(commander.clone()).await;
    // create task
    tokio::spawn(async move {
        sse_message(tx, commander.clone()).await;
    });
    // SSE
    let stream = rx.map(|msg| sse_text(msg));
    SseKeepAlive::new(stream).stream(res);
}

#[cfg(test)]
mod test {
    #[test]
    fn generate_true_command() {
        let command_name = "ping".to_string();
        let command_args = vec!["127.0.0.1".to_string()];
        let true_command = super::generate_true_command(command_name, command_args);
        assert_eq!(true_command, "ping 127.0.0.1 -c 4");
    }
}
