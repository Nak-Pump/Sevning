use salvo::prelude::TcpListener;
use salvo::{handler, Listener, Router, Server};
use Sevning::config;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut config = config::SConfig::new();
    config.init();
    let ip = config.config.application.ip.as_str();
    let port = config.config.application.port.parse::<u16>().unwrap();


    let router = Router::new()
        .path("").get(hello);


    let acceptor = TcpListener::new(
        (ip, port)).
        bind().await;
    Server::new(acceptor).serve(router).await;
}