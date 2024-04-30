use std::net::SocketAddr;

use axum::{
    extract::{Path, State},
    response::Response,
    routing::get,
    Router,
};
use tokio::net::TcpListener;

mod config;

#[tokio::main]
async fn main() {
    let c = file_server::parse_args().await;
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
        .with_state(c.server.path);
    axum::serve(tcp_listener, app).await.unwrap();
}

async fn file_server_handler(
    Path(uri_path): Path<String>,
    State(file_path): State<String>,
) -> Response {
    let path = file_path + &uri_path;
    if path.ends_with("/") {
        let file_list = file_server::local_file::FileEntry::get_files(&path).await;
        Response::new(file_server::web_page::gen_page(file_list).into())
    } else {
        let file = tokio::fs::read(&path).await.unwrap();
        Response::new(file.into())
    }
}
// while you visit http://host:port/
async fn root_file_server_handler(State(file_path): State<String>) -> Response {
    let file_list = file_server::local_file::FileEntry::get_files(&file_path).await;
    Response::new(file_server::web_page::gen_page(file_list).into())
}
