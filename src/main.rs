use salvo::prelude::TcpListener;
use salvo::{handler, Listener, Router, Server};
use Sevning::config;
use Sevning::handle::{hello_handler, sevning_handler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut config = config::SConfig::new();
    config.init();
    let ip = config.config.application.ip.as_str();
    let port = config.config.application.port.parse::<u16>().unwrap();


    let mut router = Router::new()
        .push(Router::new().path("").get(hello_handler))
        .push(Router::new().path("sevning").get(sevning_handler));

    let acceptor = TcpListener::new(
        (ip, port)).
        bind().await;
    Server::new(acceptor).serve(router).await;
}