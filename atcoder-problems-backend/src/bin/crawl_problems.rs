use atcoder_client::AtCoderClient;
use atcoder_problems_backend::crawler::ProblemCrawler;
use atcoder_problems_backend::utils::init_log_config;
use sql_client::initialize_pool;
use std::env;

#[actix_web::main]
async fn main() {
    init_log_config().unwrap();
    log::info!("Started");
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

    let db = initialize_pool(&url).await.unwrap();
    let crawler = ProblemCrawler::new(db, AtCoderClient::default());
    crawler.crawl().await.expect("Failed to crawl");

    log::info!("Finished");
}
