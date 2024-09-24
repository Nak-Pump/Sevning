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

#[handler]
pub async fn sevning_handler(req: &mut Request, res: &mut Response) {
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::spawn(send_sse_message_demo(tx));

    let stream = rx.map(|msg| sse_text(msg));
    SseKeepAlive::new(stream).stream(res);
}
