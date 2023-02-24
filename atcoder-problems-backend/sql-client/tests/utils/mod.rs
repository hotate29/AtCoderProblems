use sql_client::PgPool;
use sqlx::Executor;
use tokio::fs;

const SQL_FILE: &str = "../../config/database-definition.sql";
const DATABASE_URL_ENV_KEY: &str = "DATABASE_URL";

#[cfg(test)]
#[allow(dead_code)]
pub async fn setup_internal_user(pool: &PgPool, internal_user_id: &str, atcoder_user_id: &str) {
    sqlx::query(
        r"
        INSERT INTO internal_users (internal_user_id, atcoder_user_id)
        VALUES ($1, $2)
        ",
    )
    .bind(internal_user_id)
    .bind(atcoder_user_id)
    .execute(pool)
    .await
    .unwrap();
}

pub async fn initialize_and_connect_to_test_sql() -> PgPool {
    let database_url = std::env::var(DATABASE_URL_ENV_KEY).unwrap();
    let pool = sql_client::initialize_pool(database_url).await.unwrap();
    initialize(&pool).await;
    pool
}

async fn initialize(pool: &PgPool) {
    let sql = fs::read_to_string(SQL_FILE).await.unwrap();
    let mut conn = pool.acquire().await.unwrap();
    conn.execute(sql.as_str()).await.unwrap();
}
