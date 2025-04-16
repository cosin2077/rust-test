use axum::{
    self,
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

use clap::Parser;
use file_utils::{analyze_path, find_word, Analysis, FileStats};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "8080")]
    port: u16,
}

#[derive(Deserialize)]
struct AnalyzeQuery {
    path: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    path: String,
    word: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn analyze(Query(params): Query<AnalyzeQuery>) -> impl IntoResponse {
    match analyze_path(&params.path).await {
        Ok(analysis) => (
            StatusCode::OK,
            Json(serde_json::to_value(analysis).unwrap_or(json!({}))),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": e.to_string(),
            })),
        ),
    }
}
async fn search(Query(params): Query<SearchQuery>) -> impl IntoResponse {
    match analyze_path(&params.path).await {
        Ok(analysis) => {
            let stats = analysis.files.into_iter().find(|f| f.path == params.path);
            match stats {
                Some(stats) => match find_word(&stats, &params.word) {
                    Some(count) => (
                        StatusCode::OK,
                        Json(json!({
                            "word": params.word,
                            "count": count
                        })),
                    ),
                    None => (
                        StatusCode::NOT_FOUND,
                        Json(json!({
                            "error": format!("Word '{}' not found", params.word),
                        })),
                    ),
                },
                None => (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "error": "File not found".to_string(),
                    })),
                ),
            }
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": e.to_string()
            })),
        ),
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let app = Router::new()
        .route("/analyze", get(analyze))
        .route("/search", get(search));
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("server running on http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}

mod file_utils;
