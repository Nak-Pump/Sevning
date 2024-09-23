use salvo::prelude::TcpListener;
use salvo::{handler, Listener, Router, Server};
#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .path("").get(hello);


    let acceptor = TcpListener::new("127.0.0.1:10430").bind().await;
    Server::new(acceptor).serve(router).await;
}