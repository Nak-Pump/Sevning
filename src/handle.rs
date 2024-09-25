use std::convert::Infallible;
use std::time::Duration;
use futures_util::StreamExt;
use salvo::prelude::*;
use salvo::sse::{self, SseEvent};
use tokio::sync::mpsc;
use tokio::time::interval;
use tokio_stream::wrappers::{IntervalStream, UnboundedReceiverStream};

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
    for i in 1..command_args.len() + 1 {
        command_line = command_line.replace(&format!("{{{}}}", i), &command_args[i-1]);
    }
    command_line
}


#[handler]
pub async fn sevning_handler(req: &mut Request, res: &mut Response) {
    let mut token = req.query("token").unwrap_or("default").to_string();
    let mut command_name = req.query("command_name").unwrap_or("default").to_string();
    let mut command_args = req.query("command_args").unwrap_or("default").to_string();

    let mut command_args_vec = parse_command_args(command_args);

    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::spawn(send_sse_message_demo(tx));

    let stream = rx.map(|msg| sse_text(msg));
    SseKeepAlive::new(stream).stream(res);
}
