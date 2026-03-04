use axum::body::Body;
use axum::http::{HeaderValue, Response, StatusCode, header};
use axum::response::IntoResponse;
use axum_extra::routing::TypedPath;
use octo_assets::files::StaticFile;
use serde::Deserialize;
use tokio_util::io::ReaderStream;

#[derive(TypedPath, Deserialize)]
#[typed_path("/static/{*path}")]
pub struct StaticFilePath {
    pub path: String,
}

pub async fn static_path(StaticFilePath { path }: StaticFilePath) -> impl IntoResponse {
    let path = format!("/static/{path}");

    let data = StaticFile::get(&path);
    if let Some(data) = data {
        let file = match tokio::fs::File::open(data.file_name).await {
            Ok(file) => file,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap();
            }
        };

        let stream = ReaderStream::new(file);
        return Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime).unwrap(),
            )
            .header(
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=31536000, immutable"),
            )
            .body(Body::from_stream(stream))
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
