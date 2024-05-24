use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::{Path, Request, State},
    middleware::{from_fn, Next},
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
        .layer(from_fn(add_content_type))
        .with_state(c.server.path);
    axum::serve(tcp_listener, app).await.unwrap();
}
async fn add_content_type(req: Request, next: Next) -> Response {
    if req.uri().path().ends_with("mp4") {
        let content_type = "video/mp4";
        let mut response = next.run(req).await;
        response
            .headers_mut()
            .append("Content-Type", content_type.parse().unwrap());
        response
    } else {
        next.run(req).await
    }
}
async fn file_server_handler(
    Path(uri_path): Path<String>,
    State(file_path): State<String>,
) -> Response {
    //println!("file_path is: {:?},uri_path is: {:?}", file_path, uri_path);
    let mut path = PathBuf::from(file_path);
    path.push(&uri_path);
    //println!("39: now path is: {:?}", path);
    //file_path.extend(uri_path);
    if tokio::fs::metadata(&path).await.unwrap().is_dir() {
        // if !path.ends_with("/") {
        //     //path.push("/");
        //     println!("44: now path is: {:?}", path);
        //     //let new_path = uri_path.clone() + "/";
        //     let new_path = path
        //         .file_name()
        //         .unwrap()
        //         .to_os_string()
        //         .into_string()
        //         .unwrap();
        //     return Redirect::permanent(&new_path).into_response();
        // }
        let file_list = file_server::local_file::FileEntry::get_files(&path).await;
        Response::new(file_server::web_page::gen_page(file_list).into())
    }
    // else if path.extension().unwrap() == "mp4" {
    //     Response::new(file_server::web_page::gen_player(PathBuf::from(&uri_path)).into())
    // }
    else {
        let file = tokio::fs::read(&path).await.unwrap();
        Response::new(file.into())
    }
}
// while you visit http://host:port/
async fn root_file_server_handler(State(file_path): State<String>) -> Response {
    //println!("root path is {file_path}");
    let file_path = PathBuf::from(file_path);
    let file_list = file_server::local_file::FileEntry::get_files(&file_path).await;
    Response::new(file_server::web_page::gen_page(file_list).into())
}
