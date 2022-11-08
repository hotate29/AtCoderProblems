use atcoder_problems_backend::server::middleware::github_auth::GithubToken;
use httpmock::MockServer;
use serde_json::json;
use sql_client::{initialize_pool, PgPool};
use sqlx::Executor;
use std::fs::read_to_string;

const SQL_FILE: &str = "../config/database-definition.sql";
const DATABASE_URL_ENV_KEY: &str = "DATABASE_URL";

pub fn get_database_url_from_env() -> String {
    std::env::var(DATABASE_URL_ENV_KEY).unwrap()
}

pub async fn initialize_and_connect_to_test_sql() -> PgPool {
    let conn = initialize_pool(get_database_url_from_env()).await.unwrap();
    initialize(&conn).await;
    conn
}

pub async fn initialize(pool: &PgPool) {
    let query = read_to_string(SQL_FILE).unwrap();
    pool.execute(query.as_str()).await.unwrap();
}

pub fn start_mock_github_server(access_token: &str) -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method("POST").path("/login/oauth/access_token");
        then.status(200)
            .json_body(json!({ "access_token": access_token }));
    });
    server
}

pub fn start_mock_github_api_server(access_token: &str, token: GithubToken) -> MockServer {
    let server = MockServer::start();
    let token_header = format!("token {}", access_token);
    server.mock(|when, then| {
        when.method("GET")
            .path("/user")
            .header("Authorization", &token_header);
        then.status(200).json_body_obj(&token);
    });
    server
}
