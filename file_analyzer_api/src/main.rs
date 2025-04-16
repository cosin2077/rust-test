use axum::{
    self,
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};

use clap::Parser;
use file_utils::{analyze_path, find_word};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::io;

mod db;
mod file_utils;

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "8080")]
    port: u16,
    #[clap(
        long,
        default_value = "postgresql://postgres:2010@localhost/file_analyzer"
    )]
    database_url: String,
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
#[derive(Deserialize)]
struct SaveQuery {
    path: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
#[derive(Clone)]
struct AppState {
    db: PgPool,
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
async fn home() -> impl IntoResponse {
    let routes = vec![
        "GET  /analyze",
        "GET  /search",
        "POST /save",
        "GET  /history",
    ];
    let list_items: String = routes
        .into_iter()
        .map(|route| {
            let mut txt = route.split_whitespace();
            let mut method = txt.next().unwrap_or("");
            let mut link = txt.next().unwrap_or("");
            println!("method: {}", method);
            println!("link: {}", link);
            let after_format = format!(
                "<li>{}<a href='{}'>{}</a></li>",
                if method == "GET" {
                    format!("{}{}", method, "&nbsp;".repeat(10))
                } else {
                    format!("{}{}", method, "&nbsp;".repeat(8))
                },
                link,
                link
            );
            println!("after_format: {}", after_format);
            after_format
        })
        .collect::<Vec<String>>()
        .join("\n");
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Routes</title>
        </head>
        <body>
            <h1>Available Routes</h1>
            <ul>
            {}
            </ul>
        </body>
        </html>
        "#,
        list_items,
    ))
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
async fn save(Query(params): Query<SaveQuery>, State(state): State<AppState>) -> impl IntoResponse {
    match analyze_path(&params.path).await {
        Ok(analysis) => match db::save_analysis(&state.db, &analysis).await {
            Ok(()) => (StatusCode::OK, Json(json!({ "status": "Analysis saved"}))),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            ),
        },
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        ),
    }
}
async fn history(
    Query(params): Query<AnalyzeQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match db::get_history(&state.db, &params.path).await {
        Ok(history) => (
            StatusCode::OK,
            Json(serde_json::to_value(history).unwrap_or(json!({}))),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    let pool = db::init_pool(&args.database_url)
        .await
        .expect("Failed to connect to database");

    let app = Router::new()
        .route("/", get(home))
        .route("/analyze", get(analyze))
        .route("/search", get(search))
        .route("/save", post(save))
        .route("/history", get(history))
        .with_state(AppState { db: pool });

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    println!("server running on http://{}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await?,
        app.into_make_service(),
    )
    .await?;
    Ok(())
}
