pub type DbPool = sqlx::AnyPool;

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub sub: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

pub async fn open(database_url: &str) -> Result<DbPool, sqlx::Error> {
    sqlx::any::install_default_drivers();
    let pool = sqlx::AnyPool::connect(database_url).await?;
    run_migrations(&pool, database_url).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool, database_url: &str) -> Result<(), sqlx::Error> {
    let dir = if database_url.starts_with("postgres") || database_url.starts_with("postgresql") {
        "migrations/postgres"
    } else {
        "migrations/sqlite"
    };

    let mut paths: Vec<_> = std::fs::read_dir(dir)
        .map_err(|e| sqlx::Error::Protocol(format!("cannot read {dir}: {e}")))?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|x| x.to_str()) == Some("sql"))
        .map(|e| e.path())
        .collect();

    paths.sort();

    let mut conn = pool.acquire().await?;
    for path in paths {
        let sql =
            std::fs::read_to_string(&path).map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        for stmt in sql.split(';') {
            let trimmed = stmt.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed).execute(&mut *conn).await?;
            }
        }
    }

    Ok(())
}

pub async fn upsert_user(
    pool: &DbPool,
    sub: &str,
    name: &str,
    email: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO users (sub, name, email) VALUES (?, ?, ?)
         ON CONFLICT(sub) DO UPDATE SET name = excluded.name, email = excluded.email
         RETURNING id, sub, name, email, created_at",
    )
    .bind(sub)
    .bind(name)
    .bind(email)
    .fetch_one(pool)
    .await
}
