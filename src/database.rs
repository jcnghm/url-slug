use sqlx::{Row, SqlitePool};

pub async fn create_pool() -> SqlitePool {
    // Find the db file or create it if it doesn't exist
    let pool = SqlitePool::connect("sqlite://slugs.db?mode=rwc")
        .await
        .expect("Failed to connect to database");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS slugs (
            slug TEXT PRIMARY KEY,
            original_url TEXT NOT NULL,
            click_count INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    println!("Connected to SQLite database: slugs.db");
    pool
}

pub async fn insert_slug(pool: &SqlitePool, slug: &str, url: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO slugs (slug, original_url) VALUES (?, ?)")
        .bind(slug)
        .bind(url)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_url_and_increment(
    pool: &SqlitePool,
    slug: &str,
) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query("SELECT original_url FROM slugs WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = result {
        // query the slug/url and increment the click count
        sqlx::query("UPDATE slugs SET click_count = click_count + 1 WHERE slug = ?")
            .bind(slug)
            .execute(pool)
            .await?;

        let url: String = row.get("original_url");
        Ok(Some(url))
    } else {
        Ok(None)
    }
}

pub async fn get_slug_stats(
    pool: &SqlitePool,
    slug: &str,
) -> Result<Option<(String, u64)>, sqlx::Error> {
    let result = sqlx::query("SELECT original_url, click_count FROM slugs WHERE slug = ?")
        .bind(slug)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = result {
        let url: String = row.get("original_url");
        let count: i64 = row.get("click_count");
        Ok(Some((url, count as u64)))
    } else {
        Ok(None)
    }
}
