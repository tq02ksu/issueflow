use std::path::Path;

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
    let dir = migration_dir(database_url);
    let migrator = sqlx::migrate::Migrator::new(Path::new(dir))
        .await
        .map_err(|err| sqlx::Error::Migrate(Box::new(err)))?;
    migrator
        .run(pool)
        .await
        .map_err(|err| sqlx::Error::Migrate(Box::new(err)))
}

fn migration_dir(database_url: &str) -> &'static str {
    if database_url.starts_with("postgres") || database_url.starts_with("postgresql") {
        "migrations/postgres"
    } else {
        "migrations/sqlite"
    }
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
