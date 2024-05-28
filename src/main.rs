use std::net::SocketAddr;

use axum::{routing::get, Router};
use handler::{file_server_handler, root_file_server_handler};
use tokio::net::TcpListener;

mod config;
mod handler;
#[tokio::main]
async fn main() {
    let c = file_server::config::parse_args().await;
    println!("config is:\n{:?}", c);
    let addr = SocketAddr::new(
        c.server.host.parse().expect("host is invalid."),
        c.server.port.parse::<u16>().expect("port is invalid."),
    );
    let tcp_listener = TcpListener::bind(addr)
        .await
        .expect("unable to bind this address and port.");
    let app = Router::new()
        .route("/axum_demo", get(|| async { "hello world" }))
        .route("/", get(root_file_server_handler))
        .route("/*uri_path", get(file_server_handler))
        //.layer(from_fn(add_content_type))
        .with_state(c.server.path);
    axum::serve(tcp_listener, app).await.unwrap();
}
