use axum::{Router, extract::Multipart, response::Html, routing::get};

use std::{fs::File, io::Write};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(index).post(upload));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to start listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to serve 'app'!");
}

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../public/index.html"))
}

async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("failed to get next field")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }

        let file_name = field.file_name().unwrap();

        let file_path = format!("files/{}", file_name);

        let data = field.bytes().await.unwrap();

        let mut file_handle = File::create(file_path).expect("failed to open file handle");

        file_handle.write_all(&data).expect("Failed to write data");
    }
}
