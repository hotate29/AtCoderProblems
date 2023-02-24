use actix_web::rt::time;
use anyhow::Result;
use atcoder_client::AtCoderClient;
use atcoder_problems_backend::crawler::WholeContestCrawler;
use atcoder_problems_backend::utils::init_log_config;
use log::info;
use sql_client::initialize_pool;
use sql_client::simple_client::SimpleClient;
use std::{env, time::Duration};

const NEW_CONTEST_NUM: usize = 5;

async fn iteration(url: &str) -> Result<()> {
    let db = initialize_pool(&url).await?;
    let mut contests = db.load_contests().await?;
    contests.sort_by_key(|c| c.start_epoch_second);
    contests.reverse();

    for contest in &contests[0..NEW_CONTEST_NUM] {
        info!("Starting {}", contest.id);
        let crawler = WholeContestCrawler::new(db.clone(), AtCoderClient::default(), &contest.id);
        crawler.crawl().await?;
    }
    Ok(())
}

#[actix_web::main]
async fn main() {
    init_log_config().unwrap();
    info!("Started");
    let url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

    loop {
        info!("Start new loop");
        if let Err(e) = iteration(&url).await {
            log::error!("{:?}", e);
            time::sleep(Duration::from_secs(1)).await;
        }
    }
}
