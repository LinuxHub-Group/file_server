use std::path::PathBuf;

use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use file_server::mime::mime_from_name;

pub async fn add_content_type(req: Request, next: Next) -> Response {
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
pub async fn file_server_handler(
    Path(uri_path): Path<String>,
    State(file_path): State<String>,
) -> Response {
    //println!("file_path is: {:?},uri_path is: {:?}", file_path, uri_path);
    let mut path = PathBuf::from(file_path);
    path.push(&uri_path);
    if let Ok(file_metadata) = tokio::fs::metadata(&path).await {
        if file_metadata.is_dir() {
            let file_list = file_server::local_file::FileEntry::get_files(&path).await;
            Response::new(file_server::web_page::gen_page(file_list).into())
        } else {
            // add Content-Type header
            let file = tokio::fs::read(&path).await.unwrap();
            let mut response = Response::new(file.into());
            //let file_ext = path.extension().unwrap().to_str().unwrap();
            if let Some(file_ext) = path.extension() {
                if let Some(content_type) = mime_from_name(file_ext.to_str().unwrap()) {
                    response
                        .headers_mut()
                        .append("Content-Type", content_type.parse().unwrap());
                }
            }
            response
        }
    } else {
        (StatusCode::NOT_FOUND, Body::empty()).into_response()
    }
}
// while you visit http://host:port/
pub async fn root_file_server_handler(State(file_path): State<String>) -> Response {
    //println!("root path is {file_path}");
    let file_path = PathBuf::from(file_path);
    let file_list = file_server::local_file::FileEntry::get_files(&file_path).await;
    Response::new(file_server::web_page::gen_page(file_list).into())
}
