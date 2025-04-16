use crate::file_utils::{Analysis, DbFileStats};
use chrono::Utc;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use time::OffsetDateTime;

pub async fn init_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    Pool::connect(database_url).await
}

pub async fn save_analysis(pool: &Pool<Postgres>, analysis: &Analysis) -> Result<(), sqlx::Error> {
    for stats in &analysis.files {
        sqlx::query!(
            r#"
            INSERT INTO file_stats(path, word_count, line_count,char_count,unique_words, word_frequency,analyzed_at)
            VALUES ($1,$2,$3,$4,$5,$6,$7)
            "#,
            stats.path,
            stats.word_count as i32,
            stats.line_count as i32,
            stats.char_count as i32,
            stats.unique_words as i32,
            serde_json::to_value(&stats.word_frequency).unwrap(),
            OffsetDateTime::now_utc()
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn get_history(
    pool: &Pool<Postgres>,
    path: &str,
) -> Result<Vec<DbFileStats>, sqlx::Error> {
    sqlx::query_as::<_, DbFileStats>(
        r#"
        SELECT id, path, word_count, line_count, char_count, unique_words, word_frequency, analyzed_at
        FROM file_stats
        WHERE path = $1
        ORDER BY analyzed_at DESC
        "#,
    )
    .bind(path)
    .fetch_all(pool)
    .await
}
